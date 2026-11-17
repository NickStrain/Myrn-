//! Append-only episode logs in the private store: `episodes/<session>.jsonl`.
//!
//! One line per normalized event. The log holds what the agent did (prompts,
//! files, commands, errors, outcomes), never the model transcript. Every text
//! field is redacted by the engine before it reaches this module.

use std::io::{BufRead, Write};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

/// One line of an episode log.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Entry {
    /// Unix time in seconds.
    pub t: i64,
    /// Adapter name: `claude`, `codex`.
    pub agent: String,
    /// `session_start`, `prompt`, `tool`, `session_end`.
    pub event: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// Prompt text.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool: Option<String>,
    /// `failure`, `file_edit`, `other`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outcome: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Repo-relative paths.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub files: Vec<String>,
    /// Symbols whose body changed in this edit.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub symbols: Vec<String>,
    /// `pass` or `fail` when the command was a test run.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub test: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// How an edit was made when not through an edit tool: `shell`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub via: Option<String>,
    /// Commits made since the session started (`sha summary`).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub commits: Vec<String>,
}

/// File name for a session id. Ids come from agents, so anything outside a
/// safe set is replaced to keep the file inside the episodes directory.
pub fn file_name(session: &str) -> String {
    let mut s: String = session
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '_'
            }
        })
        .take(128)
        .collect();
    if s.is_empty() {
        s.push_str("unknown");
    }
    format!("{s}.jsonl")
}

pub fn path_for(dir: &Path, session: &str) -> PathBuf {
    dir.join(file_name(session))
}

/// Append one entry. Each write is a single line, so concurrent hooks don't interleave.
pub fn append(dir: &Path, session: &str, entry: &Entry) -> Result<PathBuf> {
    std::fs::create_dir_all(dir)?;
    let path = path_for(dir, session);
    let mut line = serde_json::to_string(entry)?;
    line.push('\n');
    let mut f = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .with_context(|| format!("opening {}", path.display()))?;
    f.write_all(line.as_bytes())?;
    Ok(path)
}

/// Read every entry; malformed lines (from a crash mid-write) are skipped.
pub fn read(path: &Path) -> Result<Vec<Entry>> {
    let f = std::fs::File::open(path).with_context(|| format!("opening {}", path.display()))?;
    Ok(std::io::BufReader::new(f)
        .lines()
        .map_while(|l| l.ok())
        .filter_map(|l| serde_json::from_str(&l).ok())
        .collect())
}

/// Session ids and log paths, newest first by modification time.
pub fn list(dir: &Path) -> Result<Vec<(String, PathBuf)>> {
    let Ok(rd) = std::fs::read_dir(dir) else {
        return Ok(Vec::new());
    };
    let mut out: Vec<(std::time::SystemTime, String, PathBuf)> = rd
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "jsonl"))
        .filter_map(|p| {
            let id = p.file_stem()?.to_string_lossy().into_owned();
            let m = p.metadata().and_then(|m| m.modified()).ok()?;
            Some((m, id, p))
        })
        .collect();
    out.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
    Ok(out.into_iter().map(|(_, id, p)| (id, p)).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn session_ids_cannot_escape_the_directory() {
        assert_eq!(file_name("../../etc/passwd"), "______etc_passwd.jsonl");
        assert_eq!(file_name(""), "unknown.jsonl");
        assert_eq!(file_name("abc-123_x"), "abc-123_x.jsonl");
    }

    #[test]
    fn append_and_read_round_trip() {
        let dir = tempfile::tempdir().unwrap();
        let e = Entry {
            t: 1,
            agent: "claude".into(),
            event: "prompt".into(),
            text: Some("fix it".into()),
            ..Default::default()
        };
        append(dir.path(), "s1", &e).unwrap();
        append(dir.path(), "s1", &e).unwrap();
        let p = path_for(dir.path(), "s1");
        std::fs::OpenOptions::new()
            .append(true)
            .open(&p)
            .unwrap()
            .write_all(b"{broken\n")
            .unwrap();
        assert_eq!(read(&p).unwrap(), vec![e.clone(), e]);
        assert_eq!(list(dir.path()).unwrap()[0].0, "s1");
    }
}
