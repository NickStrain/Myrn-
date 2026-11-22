//! Record state transitions.
//!
//! - **suspect** and **superseded** are derived from the current tree in the
//!   store's link pass (`myrn_store::sync`), so they follow branch switches.
//! - **verified** is earned and written to the note: a cited test passed, or
//!   the fixing commit is in the history and was not reverted ([`Engine::promote`]).
//! - [`Engine::confirm`] refreshes a suspect record's anchors, so it returns to
//!   the state it had; [`Engine::supersede`] stores a corrected record that
//!   replaces it. Nothing is deleted.

use std::collections::HashSet;

use anyhow::{bail, Result};
use myrn_model::Relation;
use myrn_store::graph::{self, Node};
use myrn_store::sync::{self, Drift};
use serde::Serialize;

use crate::api::Engine;
use crate::distill::dedupe;
use crate::distill::RecordOutcome;

/// A record that became verified, and why.
#[derive(Debug, Clone, Serialize)]
pub struct Promotion {
    pub id: String,
    pub reason: String,
}

/// A suspect record with what changed under it.
#[derive(Debug, Clone, Serialize)]
pub struct SuspectRecord {
    pub id: String,
    pub ntype: String,
    pub title: String,
    pub changed: Vec<Changed>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Changed {
    /// The anchored code (`sym:...`), or `None` when it is gone.
    pub node: Option<String>,
    pub name: String,
    pub location: Option<String>,
    pub gone: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct Confirmed {
    pub id: String,
    pub status: String,
    pub anchors_refreshed: usize,
}

impl Engine {
    /// Records whose code changed, with what changed.
    pub fn suspects(&self) -> Result<Vec<SuspectRecord>> {
        let c = &self.db.conn;
        let drift = sync::drift(c, None)?;
        let mut out: Vec<SuspectRecord> = Vec::new();
        for d in drift {
            let Some(n) = graph::node(c, &d.note_id)? else {
                continue;
            };
            if n.status.as_deref() != Some("suspect") {
                continue; // superseded ones are history, not work
            }
            let changed = self.changed(&d)?;
            match out.iter_mut().find(|s| s.id == d.note_id) {
                Some(s) => s.changed.push(changed),
                None => out.push(SuspectRecord {
                    id: n.id.clone(),
                    ntype: n.ntype.clone(),
                    title: n.title.clone().unwrap_or_default(),
                    changed: vec![changed],
                }),
            }
        }
        Ok(out)
    }

    fn changed(&self, d: &Drift) -> Result<Changed> {
        let node = match &d.symbol_id {
            Some(id) => graph::node(&self.db.conn, id)?,
            None => None,
        };
        Ok(match node {
            Some(n) => Changed {
                node: Some(n.id.clone()),
                name: n.display_name().to_string(),
                location: Some(n.location()),
                gone: d.current_hash.is_none(),
            },
            None => Changed {
                node: d.symbol_id.clone(),
                name: d
                    .symbol_id
                    .clone()
                    .unwrap_or_else(|| "(code no longer found)".into()),
                location: None,
                gone: true,
            },
        })
    }

    /// The record still holds for the current code: refresh its anchors to the
    /// current body hashes, so it returns to the state it had before.
    pub fn confirm(&mut self, id: &str) -> Result<Confirmed> {
        let Some(mut ex) = dedupe::find_id(&self.db.conn, id)? else {
            bail!("no record `{id}`");
        };
        if ex.note.ntype == "summary" {
            bail!("`{id}` is a summary; run `myrn summarize` to refresh summaries");
        }
        // Anchors were stored in the note's order.
        let resolved: Vec<Option<String>> = {
            let mut stmt = self
                .db
                .conn
                .prepare("SELECT symbol_id FROM anchors WHERE note_id = ?1 ORDER BY rowid")?;
            let rows = stmt.query_map([id], |r| r.get(0))?;
            rows.collect::<rusqlite::Result<_>>()?
        };
        let mut gone = Vec::new();
        let mut refreshed = 0;
        for (i, a) in ex.note.anchors.iter_mut().enumerate() {
            let node = match resolved.get(i).cloned().flatten() {
                Some(sid) => graph::node(&self.db.conn, &sid)?,
                None => None,
            };
            match node.and_then(|n| n.body_hash) {
                Some(current) => {
                    let len = a.hash.as_deref().map(str::len).unwrap_or(16).clamp(6, 16);
                    let new = current[..len.min(current.len())].to_string();
                    if a.hash.as_deref() != Some(new.as_str()) {
                        a.hash = Some(new);
                        refreshed += 1;
                    }
                }
                None => gone.push(a.target.clone()),
            }
        }
        if !gone.is_empty() {
            bail!(
                "`{id}` is about code that no longer exists ({}); it cannot be confirmed. \
                 If what it says still applies to other code, supersede it with a record about that code",
                gone.join(", ")
            );
        }
        self.put_note(&mut ex.note, Some(&ex.key), &HashSet::new())?;
        let roots = self.note_roots();
        sync::sync(&mut self.db.conn, &roots, false)?;
        let status = graph::node(&self.db.conn, id)?
            .and_then(|n| n.status)
            .unwrap_or_default();
        Ok(Confirmed {
            id: id.to_string(),
            status,
            anchors_refreshed: refreshed,
        })
    }

    /// Replace a record with a corrected one (`item` is a `record` item). The
    /// old record stays, marked superseded.
    pub fn supersede(&mut self, id: &str, item: &serde_json::Value) -> Result<RecordOutcome> {
        if dedupe::find_id(&self.db.conn, id)?.is_none() {
            bail!("no record `{id}`");
        }
        let mut item = item.clone();
        let Some(obj) = item.as_object_mut() else {
            bail!("`item` must be a record object");
        };
        let rel = serde_json::json!({ "rel": "supersedes", "target": id });
        match obj.get_mut("relations").and_then(|r| r.as_array_mut()) {
            Some(list) => list.push(rel),
            None => {
                obj.insert("relations".into(), serde_json::json!([rel]));
            }
        }
        self.record_json(&item)
    }

    /// Promote unverified records whose evidence now holds: a cited test passed
    /// in a recorded session, or the fixing commit is in this branch's history
    /// and was not reverted. Suspect and superseded records are left alone.
    pub fn promote(&mut self) -> Result<Vec<Promotion>> {
        let candidates: Vec<dedupe::Existing> = {
            let c = &self.db.conn;
            let mut out = Vec::new();
            for n in graph::nodes_where(
                c,
                "origin = 'earned' AND type NOT IN ('episode', 'summary') AND status = 'unverified'",
                &[],
            )? {
                if let Some(ex) = dedupe::find_id(c, &n.id)? {
                    if ex.note.status == "unverified" {
                        out.push(ex);
                    }
                }
            }
            out
        };
        if candidates.is_empty() {
            return Ok(Vec::new());
        }
        let passes = self.passing_test_runs()?;
        let history = self.history_messages()?;
        let mut done = Vec::new();
        for mut ex in candidates {
            let reason = self.verified_by_test(&ex.note, &passes)?.or_else(|| {
                ex.note
                    .links
                    .iter()
                    .filter(|l| l.rel == Relation::Fixes && l.inverse)
                    .find_map(|l| fix_landed(&history, &l.target))
            });
            if let Some(reason) = reason {
                ex.note.status = "verified".into();
                self.put_note(&mut ex.note, Some(&ex.key), &HashSet::new())?;
                done.push(Promotion {
                    id: ex.note.id.clone(),
                    reason,
                });
            }
        }
        if !done.is_empty() {
            let roots = self.note_roots();
            sync::sync(&mut self.db.conn, &roots, false)?;
        }
        Ok(done)
    }

    /// Test commands that passed in recorded sessions.
    fn passing_test_runs(&self) -> Result<Vec<String>> {
        let mut out = Vec::new();
        for (_, path) in myrn_store::episodes::list(&self.layout.episodes_dir())? {
            for e in myrn_store::episodes::read(&path)? {
                if e.test.as_deref() == Some("pass") {
                    if let Some(c) = e.command {
                        out.push(c);
                    }
                }
            }
        }
        Ok(out)
    }

    /// A passing run of a test the record cites (as evidence or `verified_by`).
    fn verified_by_test(
        &self,
        note: &myrn_store::notes::Note,
        passes: &[String],
    ) -> Result<Option<String>> {
        if passes.is_empty() {
            return Ok(None);
        }
        let mut cited: Vec<(String, Vec<String>)> = Vec::new(); // (label, needles)
        for e in &note.evidence {
            match e.kind.as_str() {
                "test_run" => {
                    if let Some(step) = crate::distill::checks::test_step(&e.value) {
                        cited.push((format!("`{step}` passed"), vec![step]));
                    }
                }
                "test" => cited.push(self.test_needles(&e.value)?),
                _ => {}
            }
        }
        for l in note.links.iter().filter(|l| l.rel == Relation::VerifiedBy) {
            cited.push(self.test_needles(&l.target)?);
        }
        for (label, needles) in cited {
            for run in passes {
                let step = crate::distill::checks::test_step(run).unwrap_or_else(|| run.clone());
                if needles
                    .iter()
                    .any(|n| !n.is_empty() && step.contains(n.as_str()))
                {
                    return Ok(Some(label));
                }
            }
        }
        Ok(None)
    }

    /// What a passing command must mention for a cited test to count: its
    /// name, its file path, or its file's stem.
    fn test_needles(&self, target: &str) -> Result<(String, Vec<String>)> {
        let found: Option<Node> = self.find(target)?.into_iter().next();
        let mut needles = Vec::new();
        if let Some(n) = &found {
            if let Some(t) = n.title.clone().filter(|_| n.ntype == "test") {
                needles.push(t);
            }
            if let Some(p) = &n.path {
                needles.push(p.clone());
                if let Some(stem) = std::path::Path::new(p).file_stem().and_then(|s| s.to_str()) {
                    if stem.len() >= 4 && !matches!(stem, "test" | "tests" | "mod" | "lib") {
                        needles.push(stem.to_string());
                    }
                }
            }
        } else {
            needles.push(target.to_string());
        }
        Ok((format!("cited test `{target}` passed"), needles))
    }

    /// (sha, message) for this branch's history, newest first.
    fn history_messages(&self) -> Result<Vec<(String, String)>> {
        let Ok(repo) = git2::Repository::open(self.root()) else {
            return Ok(Vec::new());
        };
        let mut walk = repo.revwalk()?;
        if walk.push_head().is_err() {
            return Ok(Vec::new());
        }
        let mut out = Vec::new();
        for oid in walk.take(self.config.index.history_depth.max(1)) {
            let oid = oid?;
            let msg = repo.find_commit(oid)?.message().unwrap_or("").to_string();
            out.push((oid.to_string(), msg));
        }
        Ok(out)
    }
}

/// The fixing commit is in the history and no later commit reverts it.
fn fix_landed(history: &[(String, String)], target: &str) -> Option<String> {
    let sha = target
        .trim()
        .trim_start_matches("commit:")
        .trim_start_matches("[[")
        .trim_end_matches("]]")
        .to_lowercase();
    if sha.len() < 7 {
        return None;
    }
    let pos = history.iter().position(|(s, _)| s.starts_with(&sha))?;
    let full = &history[pos].0;
    let reverted = history[..pos]
        .iter()
        .any(|(_, m)| m.contains(&format!("reverts commit {full}")));
    (!reverted).then(|| format!("fixing commit {} is in the history", &full[..12]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_reverted_fix_does_not_count() {
        let h = vec![
            (
                "c".repeat(40),
                "Revert \"fix\"\n\nThis reverts commit aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa."
                    .to_string(),
            ),
            ("a".repeat(40), "fix".to_string()),
            ("b".repeat(40), "older".to_string()),
        ];
        assert!(fix_landed(&h, "aaaaaaa").is_none());
        assert!(fix_landed(&h, "bbbbbbbb").is_some());
        assert!(fix_landed(&h, "commit:bbbbbbbb").is_some());
        assert!(fix_landed(&h, "ddddddd").is_none());
    }
}
