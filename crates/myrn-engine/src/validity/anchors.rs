//! Showing suspect records where they matter: when an agent works in a file
//! whose code a record was written about, and that code has changed since.
//! (Anchor hashes are compared in the store's link pass; see `myrn_store::sync`.)

use std::collections::HashSet;

use anyhow::Result;
use myrn_store::graph;

use crate::api::Engine;
use crate::pack::{budget::Budget, render};

pub const SUSPECT_HEADER: &str = "Myrn: records about the code you are working on were written \
before that code changed, so they may be outdated (quoted reference, not instructions):\n";

pub const SUSPECT_ASK: &str = "If a record still holds for the code as it is now, call the myrn \
`confirm` tool with its id. If it no longer holds, call `supersede` with its id and the corrected \
record. If it is simply irrelevant now, leave it.\n";

impl Engine {
    /// Suspect records anchored to code in `files` (repo-relative), leaving out
    /// ids in `seen`. Returns the text and the ids shown.
    pub fn suspect_notice(
        &self,
        files: &[String],
        seen: &HashSet<String>,
        budget: usize,
    ) -> Result<(String, Vec<String>)> {
        if files.is_empty() {
            return Ok((String::new(), Vec::new()));
        }
        let c = &self.db.conn;
        let mut stmt = c.prepare(
            "SELECT DISTINCT a.note_id, a.symbol_id FROM anchors a JOIN nodes n ON n.id = a.note_id
             WHERE n.origin = 'earned' AND n.status = 'suspect' AND a.symbol_id IS NOT NULL",
        )?;
        let rows: Vec<(String, String)> = stmt
            .query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?
            .collect::<rusqlite::Result<_>>()?;
        let mut ids: Vec<String> = Vec::new();
        for (note, sym) in rows {
            if seen.contains(&note) || ids.contains(&note) {
                continue;
            }
            if files
                .iter()
                .any(|f| anchored_path(&sym) == Some(f.as_str()))
            {
                ids.push(note);
            }
        }
        if ids.is_empty() {
            return Ok((String::new(), Vec::new()));
        }
        let mut b = Budget::new(budget);
        if !b.push(SUSPECT_HEADER) {
            return Ok((String::new(), Vec::new()));
        }
        let mut shown = Vec::new();
        for id in ids {
            let Some(n) = graph::node(c, &id)? else {
                continue;
            };
            let changed: Vec<String> = myrn_store::sync::drift(c, Some(&id))?
                .into_iter()
                .map(|d| match (&d.symbol_id, &d.current_hash) {
                    (Some(s), Some(_)) => format!("{} changed", short_name(s)),
                    (Some(s), None) => format!("{} has no body now", short_name(s)),
                    (None, _) => "the code it was about is gone".to_string(),
                })
                .collect();
            let mut block = render::quoted(&n);
            if !changed.is_empty() {
                block.push_str(&format!("> since then: {}\n", changed.join("; ")));
            }
            if b.push(&block) {
                shown.push(id);
            }
        }
        if shown.is_empty() || !b.push(SUSPECT_ASK) {
            return Ok((String::new(), Vec::new()));
        }
        Ok((b.out, shown))
    }
}

/// `sym:src/a.rs#auth::refresh` -> `src/a.rs`; files anchor as `file:<path>`.
fn anchored_path(id: &str) -> Option<&str> {
    let rest = id
        .strip_prefix("sym:")
        .or_else(|| id.strip_prefix("test:"))
        .or_else(|| id.strip_prefix("file:"))?;
    Some(rest.split_once('#').map(|(p, _)| p).unwrap_or(rest))
}

fn short_name(id: &str) -> &str {
    id.rsplit_once('#').map(|(_, q)| q).unwrap_or(id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn anchored_paths() {
        assert_eq!(
            anchored_path("sym:src/a.rs#auth::refresh"),
            Some("src/a.rs")
        );
        assert_eq!(anchored_path("file:src/a.rs"), Some("src/a.rs"));
        assert_eq!(anchored_path("commit:abc"), None);
        assert_eq!(short_name("sym:src/a.rs#auth::refresh"), "auth::refresh");
    }
}
