//! `myrn gc`: merge duplicate note copies, drop index entries for code that
//! is gone, and compact old session logs. Never deletes an earned record: a
//! copy is removed only when another file holds the same note.

use std::collections::HashSet;

use anyhow::Result;
use myrn_store::episodes::{self, Entry};
use myrn_store::notes::frontmatter;
use serde::Serialize;

use crate::api::Engine;

#[derive(Debug, Clone, Default, Serialize)]
pub struct GcReport {
    pub dry_run: bool,
    /// Duplicate copies removed (same id and same content as the indexed copy).
    pub duplicates_removed: Vec<String>,
    /// Duplicate copies that differ from the indexed one: left for a person.
    pub duplicates_differing: Vec<String>,
    /// Index rows dropped because what they pointed at is gone.
    pub index_rows_dropped: usize,
    /// Session logs compacted, and the log lines dropped from them.
    pub episodes_compacted: usize,
    pub episode_lines_dropped: usize,
    pub bytes_before: u64,
    pub bytes_after: u64,
}

impl Engine {
    /// Tidy the store. Session logs older than `older_than_days` keep only
    /// what records and `trace` need: the prompt, edits, failures, test
    /// outcomes and commits; plain reads and other tool calls are dropped.
    pub fn gc(&mut self, older_than_days: u64, dry_run: bool) -> Result<GcReport> {
        self.refresh()?;
        let mut r = GcReport {
            dry_run,
            bytes_before: std::fs::metadata(&self.layout.db_path)
                .map(|m| m.len())
                .unwrap_or(0),
            ..Default::default()
        };

        // 1. Duplicate copies of one note.
        let dups: Vec<(String, String)> = {
            let mut stmt = self.db.conn.prepare(
                "SELECT path, note_id FROM note_issues WHERE kind = 'duplicate' ORDER BY path",
            )?;
            let rows = stmt.query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?;
            rows.collect::<rusqlite::Result<_>>()?
        };
        for (key, id) in dups {
            let Some(kept) = crate::distill::dedupe::find_id(&self.db.conn, &id)? else {
                continue;
            };
            let path = self.note_file(&key);
            let Ok(text) = std::fs::read_to_string(&path) else {
                continue;
            };
            let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
            let copy = frontmatter::parse(&text, stem, None).note;
            let same = copy
                .body
                .split_whitespace()
                .eq(kept.note.body.split_whitespace())
                && copy.links == kept.note.links
                && copy.anchors == kept.note.anchors;
            if same && kept.key != key {
                if !dry_run {
                    std::fs::remove_file(&path)?;
                }
                r.duplicates_removed.push(key);
            } else if kept.key != key {
                r.duplicates_differing.push(key);
            }
        }

        // 2. Index rows that point at nothing.
        let c = &self.db.conn;
        let checks = [
            "search WHERE rowid NOT IN (SELECT rowid FROM nodes)",
            "summary_search WHERE id NOT IN (SELECT id FROM nodes)",
            "edges WHERE src NOT IN (SELECT id FROM nodes) OR dst NOT IN (SELECT id FROM nodes)",
            "usage WHERE note_id NOT IN (SELECT id FROM nodes)",
            "refs WHERE path NOT IN (SELECT path FROM files)",
            "commit_files WHERE sha NOT IN (SELECT substr(id, 8) FROM nodes WHERE type = 'commit')",
        ];
        for t in checks {
            let n: i64 = c.query_row(&format!("SELECT COUNT(*) FROM {t}"), [], |r| r.get(0))?;
            r.index_rows_dropped += n as usize;
            if !dry_run && n > 0 {
                c.execute(&format!("DELETE FROM {t}"), [])?;
            }
        }

        // 3. Old session logs.
        let cutoff = now() - (older_than_days as i64) * 86_400;
        for (_, path) in episodes::list(&self.layout.episodes_dir())? {
            let entries = episodes::read(&path)?;
            let last = entries.iter().map(|e| e.t).max().unwrap_or(0);
            if last == 0 || last > cutoff {
                continue;
            }
            let kept = compact(&entries);
            if kept.len() == entries.len() {
                continue;
            }
            r.episodes_compacted += 1;
            r.episode_lines_dropped += entries.len() - kept.len();
            if !dry_run {
                let mut text = String::new();
                for e in &kept {
                    text.push_str(&serde_json::to_string(e)?);
                    text.push('\n');
                }
                let tmp = path.with_extension("jsonl.tmp");
                std::fs::write(&tmp, text)?;
                std::fs::rename(&tmp, &path)?;
            }
        }

        if !dry_run {
            if !r.duplicates_removed.is_empty() {
                let roots = self.note_roots();
                myrn_store::sync::sync(&mut self.db.conn, &roots, true)?;
            }
            if r.episodes_compacted > 0 {
                self.reindex_episodes()?;
            }
            self.db
                .conn
                .execute_batch("PRAGMA wal_checkpoint(TRUNCATE); VACUUM;")?;
        }
        r.bytes_after = std::fs::metadata(&self.layout.db_path)
            .map(|m| m.len())
            .unwrap_or(0);
        Ok(r)
    }
}

/// The lines of a session log worth keeping once it is old.
fn compact(entries: &[Entry]) -> Vec<Entry> {
    let mut out = Vec::new();
    let mut prompts = 0;
    let mut reads: Vec<String> = Vec::new();
    let mut seen_reads = HashSet::new();
    for e in entries {
        let keep = match e.event.as_str() {
            "session_start" | "session_end" => true,
            "prompt" => {
                prompts += 1;
                prompts == 1
            }
            _ => {
                e.test.is_some()
                    || !e.commits.is_empty()
                    || matches!(e.outcome.as_deref(), Some("file_edit") | Some("failure"))
            }
        };
        if keep {
            out.push(e.clone());
        } else if e.event == "tool" {
            for f in &e.files {
                if seen_reads.insert(f.clone()) {
                    reads.push(f.clone());
                }
            }
        }
    }
    // The files it read, folded into one line.
    if !reads.is_empty() {
        let t = entries.iter().map(|e| e.t).max().unwrap_or(0);
        out.push(Entry {
            t,
            agent: entries.first().map(|e| e.agent.clone()).unwrap_or_default(),
            event: "tool".into(),
            tool: Some("read (compacted)".into()),
            outcome: Some("other".into()),
            files: reads,
            ..Default::default()
        });
    }
    if out.len() >= entries.len() {
        return entries.to_vec();
    }
    out
}

fn now() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tool(t: i64, outcome: &str, files: &[&str]) -> Entry {
        Entry {
            t,
            agent: "claude".into(),
            event: "tool".into(),
            outcome: Some(outcome.into()),
            files: files.iter().map(|s| s.to_string()).collect(),
            ..Default::default()
        }
    }

    #[test]
    fn compaction_keeps_what_matters() {
        let mut test_run = tool(5, "other", &[]);
        test_run.test = Some("pass".into());
        let entries = vec![
            Entry {
                t: 1,
                event: "session_start".into(),
                ..Default::default()
            },
            Entry {
                t: 2,
                event: "prompt".into(),
                text: Some("fix it".into()),
                ..Default::default()
            },
            tool(3, "other", &["a.rs"]),
            tool(3, "other", &["a.rs", "b.rs"]),
            tool(4, "file_edit", &["a.rs"]),
            Entry {
                t: 4,
                event: "prompt".into(),
                text: Some("thanks".into()),
                ..Default::default()
            },
            test_run,
        ];
        let out = compact(&entries);
        assert_eq!(out.len(), 5, "{out:?}");
        assert_eq!(out.last().unwrap().files, ["a.rs", "b.rs"]);
        assert!(out.iter().any(|e| e.test.as_deref() == Some("pass")));
        assert!(out
            .iter()
            .any(|e| e.outcome.as_deref() == Some("file_edit")));
        // Nothing to drop: unchanged.
        assert_eq!(compact(&out[..3]).len(), 3);
    }
}
