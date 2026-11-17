//! Build an Episode from the event stream: task, files read/edited, commands,
//! errors and outcome. The log file is the source of truth; the Episode node in
//! the graph is recomputed from it after every event and on `rebuild`.

use std::collections::BTreeSet;

use anyhow::Result;
use myrn_model::id::{episode_id, file_id};
use myrn_store::episodes::Entry;
use myrn_store::graph::{self, Node};
use rusqlite::{params, Connection};
use serde::Serialize;

/// Errors kept per episode, newest last.
const MAX_ERRORS: usize = 20;
/// Commands kept per episode, newest last.
const MAX_COMMANDS: usize = 50;

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct Episode {
    pub session: String,
    pub agent: String,
    pub started: i64,
    pub ended: Option<i64>,
    /// The first prompt.
    pub task: Option<String>,
    pub prompts: usize,
    pub files_read: BTreeSet<String>,
    pub files_edited: BTreeSet<String>,
    pub symbols_touched: BTreeSet<String>,
    pub commands: Vec<String>,
    pub errors: Vec<String>,
    pub tests_passed: usize,
    pub tests_failed: usize,
    /// `pass` or `fail`: the last test run.
    pub last_test: Option<String>,
    pub commits: Vec<String>,
}

impl Episode {
    /// `tests_passed`, `tests_failed`, `committed`, `edited`, or `no_changes`.
    pub fn outcome(&self) -> &'static str {
        match self.last_test.as_deref() {
            Some("pass") => "tests_passed",
            Some(_) => "tests_failed",
            None if !self.commits.is_empty() => "committed",
            None if !self.files_edited.is_empty() => "edited",
            None => "no_changes",
        }
    }

    /// One line: what happened, mechanically.
    pub fn summary(&self) -> String {
        let mut parts = Vec::new();
        let plural = |n: usize, w: &str| format!("{n} {w}{}", if n == 1 { "" } else { "s" });
        if !self.files_edited.is_empty() {
            let mut s = format!("edited {}", plural(self.files_edited.len(), "file"));
            if !self.symbols_touched.is_empty() {
                s += &format!(" ({})", plural(self.symbols_touched.len(), "symbol"));
            }
            parts.push(s);
        }
        if !self.files_read.is_empty() {
            parts.push(format!("read {}", plural(self.files_read.len(), "file")));
        }
        let runs = self.tests_passed + self.tests_failed;
        if runs > 0 {
            let last = match self.last_test.as_deref() {
                Some("pass") => "last passed",
                _ => "last failed",
            };
            parts.push(format!("{}, {last}", plural(runs, "test run")));
        }
        if !self.errors.is_empty() {
            parts.push(plural(self.errors.len(), "error"));
        }
        if !self.commits.is_empty() {
            parts.push(plural(self.commits.len(), "commit"));
        }
        if parts.is_empty() {
            "no changes".into()
        } else {
            parts.join(", ")
        }
    }
}

/// Fold a session's log into one Episode.
pub fn fold(session: &str, entries: &[Entry]) -> Episode {
    let mut ep = Episode {
        session: session.to_string(),
        ..Default::default()
    };
    for (i, e) in entries.iter().enumerate() {
        if i == 0 {
            ep.started = e.t;
            ep.agent = e.agent.clone();
        }
        match e.event.as_str() {
            "prompt" => {
                ep.prompts += 1;
                if ep.task.is_none() {
                    ep.task = e.text.clone();
                }
            }
            "tool" => {
                let edited = e.outcome.as_deref() == Some("file_edit");
                for f in &e.files {
                    if edited {
                        ep.files_edited.insert(f.clone());
                    } else {
                        ep.files_read.insert(f.clone());
                    }
                }
                ep.symbols_touched.extend(e.symbols.iter().cloned());
                if let Some(c) = &e.command {
                    ep.commands.push(c.clone());
                }
                if let Some(err) = &e.error {
                    ep.errors.push(err.clone());
                }
                match e.test.as_deref() {
                    Some("pass") => ep.tests_passed += 1,
                    Some("fail") => ep.tests_failed += 1,
                    _ => {}
                }
                if e.test.is_some() {
                    ep.last_test = e.test.clone();
                }
            }
            "session_end" => {
                ep.ended = Some(e.t);
                for c in &e.commits {
                    if !ep.commits.contains(c) {
                        ep.commits.push(c.clone());
                    }
                }
            }
            _ => {}
        }
    }
    // A file both read and edited counts as edited.
    let edited = ep.files_edited.clone();
    ep.files_read.retain(|f| !edited.contains(f));
    let skip = ep.commands.len().saturating_sub(MAX_COMMANDS);
    ep.commands.drain(..skip);
    let skip = ep.errors.len().saturating_sub(MAX_ERRORS);
    ep.errors.drain(..skip);
    ep
}

/// Write the Episode node and its `touches` edges.
pub fn persist(conn: &Connection, ep: &Episode) -> Result<()> {
    let id = episode_id(&ep.session);
    let title = ep
        .task
        .as_deref()
        .map(|t| clip(t.lines().next().unwrap_or(t), 100));
    graph::upsert_node(
        conn,
        &Node {
            id: id.clone(),
            ntype: "episode".into(),
            origin: "earned".into(),
            kind: Some(ep.agent.clone()),
            title,
            qname: None,
            summary: Some(ep.summary()),
            doc: None,
            signature: None,
            path: None,
            lang: None,
            start_line: None,
            end_line: None,
            body_hash: None,
            status: Some(ep.outcome().into()),
            extra: Some(serde_json::to_string(ep)?),
        },
    )?;
    // Only the edges computed here; `produced` edges come from the notes.
    conn.execute(
        "DELETE FROM edges WHERE src = ?1 AND rel = 'touches'",
        params![id],
    )?;
    let exists = |nid: &str| -> Result<bool> { Ok(graph::node(conn, nid)?.is_some()) };
    let mut with_symbol = BTreeSet::new();
    for s in &ep.symbols_touched {
        if exists(s)? {
            graph::insert_edge(conn, &id, "touches", s, Some("resolved"))?;
            if let Some(p) = s.split_once(':').and_then(|(_, r)| r.split_once('#')) {
                with_symbol.insert(p.0.to_string());
            }
        }
    }
    // Files edited without a changed symbol (new files, config, docs).
    for f in &ep.files_edited {
        let fid = file_id(f);
        if !with_symbol.contains(f) && exists(&fid)? {
            graph::insert_edge(conn, &id, "touches", &fid, Some("resolved"))?;
        }
    }
    Ok(())
}

pub fn clip(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        return s.to_string();
    }
    let mut out: String = s.chars().take(max.saturating_sub(1)).collect();
    out.push('…');
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn e(t: i64, event: &str) -> Entry {
        Entry {
            t,
            agent: "claude".into(),
            event: event.into(),
            ..Default::default()
        }
    }

    #[test]
    fn folds_a_session() {
        let mut p = e(2, "prompt");
        p.text = Some("fix the 401".into());
        let mut read = e(3, "tool");
        read.outcome = Some("other".into());
        read.files = vec!["src/a.rs".into(), "src/b.rs".into()];
        let mut edit = e(4, "tool");
        edit.outcome = Some("file_edit".into());
        edit.files = vec!["src/a.rs".into()];
        edit.symbols = vec!["sym:src/a.rs#f".into()];
        let mut fail = e(5, "tool");
        fail.outcome = Some("failure".into());
        fail.command = Some("cargo test".into());
        fail.error = Some("1 failed".into());
        fail.test = Some("fail".into());
        let mut pass = e(6, "tool");
        pass.outcome = Some("other".into());
        pass.command = Some("cargo test".into());
        pass.test = Some("pass".into());
        let mut end = e(7, "session_end");
        end.commits = vec!["abc fix".into()];
        let ep = fold(
            "s1",
            &[
                e(1, "session_start"),
                p,
                read,
                edit,
                fail,
                pass,
                end.clone(),
                end,
            ],
        );
        assert_eq!(ep.started, 1);
        assert_eq!(ep.ended, Some(7));
        assert_eq!(ep.task.as_deref(), Some("fix the 401"));
        assert_eq!(ep.files_edited.iter().collect::<Vec<_>>(), ["src/a.rs"]);
        assert_eq!(ep.files_read.iter().collect::<Vec<_>>(), ["src/b.rs"]);
        assert_eq!((ep.tests_failed, ep.tests_passed), (1, 1));
        assert_eq!(ep.commits, ["abc fix"]);
        assert_eq!(ep.outcome(), "tests_passed");
        assert_eq!(
            ep.summary(),
            "edited 1 file (1 symbol), read 1 file, 2 test runs, last passed, 1 error, 1 commit"
        );
    }
}
