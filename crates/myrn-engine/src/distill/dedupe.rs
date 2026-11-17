//! Match bugs by normalized error signature and touched symbols; update instead of duplicating.
//!
//! - Bug: same error signature (or nearly: same kind, most message words in
//!   common) and the same code involved, when both sides name code.
//! - Summary: one per symbol or file; a new summary replaces the old text.
//! - Anything else: the same content hashes to the same id.
//!
//! A match is merged into the existing note: evidence, links and anchors are
//! combined (anchors take the current hash) and new text is appended.

use std::collections::HashSet;

use anyhow::Result;
use myrn_store::notes::Note;
use rusqlite::{params, Connection};

use super::signature::Signature;

/// Signatures at least this alike are the same bug.
pub const SAME_BUG: f64 = 0.75;

/// An existing note: its key in the notes table and its parsed content.
pub struct Existing {
    pub key: String,
    pub note: Note,
}

fn load(conn: &Connection, sql: &str, args: &[&dyn rusqlite::ToSql]) -> Result<Vec<Existing>> {
    let mut stmt = conn.prepare(sql)?;
    let rows = stmt.query_map(args, |r| {
        Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?))
    })?;
    let mut out = Vec::new();
    for row in rows {
        let (key, data) = row?;
        if let Ok(note) = serde_json::from_str::<Note>(&data) {
            out.push(Existing { key, note });
        }
    }
    Ok(out)
}

/// `TypeError: unhashable type` back into a signature.
pub fn parse_signature(s: &str) -> Signature {
    match s.split_once(": ") {
        Some((k, m)) => Signature {
            kind: k.to_string(),
            message: m.to_string(),
        },
        None => Signature {
            kind: s.to_string(),
            message: String::new(),
        },
    }
}

/// Symbols a bug involves: its own anchors and those of its root causes.
pub fn bug_symbols(conn: &Connection, bug_id: &str) -> Result<HashSet<String>> {
    let mut stmt = conn.prepare(
        "SELECT symbol_id FROM anchors WHERE symbol_id IS NOT NULL AND (note_id = ?1 OR note_id IN
             (SELECT dst FROM edges WHERE src = ?1 AND rel = 'caused_by'))",
    )?;
    let rows = stmt.query_map(params![bug_id], |r| r.get::<_, String>(0))?;
    Ok(rows.collect::<rusqlite::Result<_>>()?)
}

/// A known bug with the same signature and overlapping code, best match first.
pub fn find_bug(
    conn: &Connection,
    sig: &Signature,
    symbols: &HashSet<String>,
) -> Result<Option<Existing>> {
    let mut best: Option<(f64, Existing)> = None;
    for e in load(
        conn,
        "SELECT path, data FROM notes WHERE type = 'bug'
         ORDER BY visibility = 'shared' DESC, path",
        &[],
    )? {
        let Some(theirs) = e.note.signature.as_deref().map(parse_signature) else {
            continue;
        };
        let sim = sig.similarity(&theirs);
        if sim < SAME_BUG {
            continue;
        }
        let known = bug_symbols(conn, &e.note.id)?;
        if !symbols.is_empty() && !known.is_empty() && symbols.is_disjoint(&known) {
            continue;
        }
        if best.as_ref().is_none_or(|(s, _)| sim > *s) {
            best = Some((sim, e));
        }
    }
    Ok(best.map(|(_, e)| e))
}

/// The summary note for a symbol or file, if there is one.
pub fn find_summary(conn: &Connection, target: &str) -> Result<Option<Existing>> {
    Ok(load(
        conn,
        "SELECT path, data FROM notes WHERE type = 'summary' AND id IN
             (SELECT note_id FROM anchors WHERE symbol_id = ?1)
         ORDER BY visibility = 'shared' DESC, path LIMIT 1",
        &[&target],
    )?
    .into_iter()
    .next())
}

/// A note with this exact id.
pub fn find_id(conn: &Connection, id: &str) -> Result<Option<Existing>> {
    Ok(load(
        conn,
        "SELECT path, data FROM notes WHERE id = ?1
         ORDER BY visibility = 'shared' DESC, path LIMIT 1",
        &[&id],
    )?
    .into_iter()
    .next())
}

/// Fold `new` into `old`.
pub fn merge(old: &mut Note, new: &Note, replace_body: bool) {
    for e in &new.evidence {
        if !old.evidence.contains(e) {
            old.evidence.push(e.clone());
        }
    }
    for l in &new.links {
        if !old.links.contains(l) {
            old.links.push(l.clone());
        }
    }
    for a in &new.anchors {
        match old.anchors.iter_mut().find(|o| o.target == a.target) {
            Some(o) => o.hash = a.hash.clone(),
            None => old.anchors.push(a.clone()),
        }
    }
    if old.signature.is_none() {
        old.signature = new.signature.clone();
    }
    if replace_body {
        old.body = new.body.clone();
        if new.title.is_some() {
            old.title = new.title.clone();
        }
    } else {
        let add = new.body.trim();
        let have: String = old.body.split_whitespace().collect::<Vec<_>>().join(" ");
        let add_norm: String = add.split_whitespace().collect::<Vec<_>>().join(" ");
        if !add.is_empty() && !have.contains(&add_norm) {
            old.body = format!("{}\n\n{add}\n", old.body.trim_end());
        }
    }
    if new.source.is_some() {
        old.source = new.source.clone();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use myrn_store::notes::{Anchor, Evidence};

    #[test]
    fn merge_combines_and_refreshes_anchors() {
        let mut old = Note::new("bug", "Refresh returns 401.\n");
        old.anchors.push(Anchor {
            target: "a::f".into(),
            hash: Some("old".into()),
        });
        let mut new = Note::new("bug", "Refresh returns 401.");
        new.anchors.push(Anchor {
            target: "a::f".into(),
            hash: Some("new".into()),
        });
        new.evidence.push(Evidence {
            kind: "test".into(),
            value: "t".into(),
        });
        merge(&mut old, &new, false);
        assert_eq!(old.anchors[0].hash.as_deref(), Some("new"));
        assert_eq!(old.evidence.len(), 1);
        assert_eq!(
            old.body, "Refresh returns 401.\n",
            "same text is not appended twice"
        );
        let mut more = Note::new("bug", "Also when the clock is skewed.");
        more.signature = Some("E: x".into());
        merge(&mut old, &more, false);
        assert!(old.body.ends_with("Also when the clock is skewed.\n"));
        assert_eq!(old.signature.as_deref(), Some("E: x"));
    }
}
