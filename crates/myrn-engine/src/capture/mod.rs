//! Session capture: normalized events into an episode log. Off until consent.
//!
//! Every event becomes one redacted log line. Paths are made repo-relative;
//! paths outside the repository or listed in `.myrn/ignore` are dropped, and an
//! event that only concerned such paths is not recorded at all.

pub mod consent;
pub mod episode;
pub mod outcome;
pub mod redact;

use std::path::Path;

use ignore::gitignore::Gitignore;
use myrn_model::event::{Event, EventKind, ToolOutcome};
use myrn_store::episodes::Entry;
use myrn_store::paths::repo_rel;

const MAX_PROMPT: usize = 4000;
const MAX_COMMAND: usize = 1000;
const MAX_ERROR: usize = 2000;

/// Turn an event into a log entry, or `None` when there is nothing worth keeping.
/// Symbols touched and commits are filled in by the engine, which can see the index.
pub fn to_entry(
    agent: &str,
    event: &Event,
    root: &Path,
    ignore: &Gitignore,
    t: i64,
) -> Option<Entry> {
    let mut e = Entry {
        t,
        agent: agent.to_string(),
        ..Default::default()
    };
    match &event.kind {
        EventKind::SessionStart { source } => {
            e.event = "session_start".into();
            e.source = source.clone();
        }
        EventKind::PromptSubmitted { prompt } => {
            e.event = "prompt".into();
            e.text = Some(clean(prompt, MAX_PROMPT, root, ignore));
        }
        EventKind::ToolResult(r) => {
            e.event = "tool".into();
            e.tool = Some(r.tool.clone());
            e.outcome = Some(
                match r.outcome {
                    ToolOutcome::Failure => "failure",
                    ToolOutcome::FileEdit => "file_edit",
                    ToolOutcome::Other => "other",
                }
                .into(),
            );
            let cwd = event.cwd.as_deref().map(Path::new).unwrap_or(root);
            let mut dropped = false;
            for f in &r.files {
                match relative(root, cwd, f) {
                    Some(rel) if !is_ignored(ignore, &rel) => {
                        let rel = redact::redact(&rel).into_owned();
                        if !e.files.contains(&rel) {
                            e.files.push(rel);
                        }
                    }
                    _ => dropped = true,
                }
            }
            if dropped && e.files.is_empty() {
                return None;
            }
            e.command = r
                .command
                .as_deref()
                .map(|c| clean(c, MAX_COMMAND, root, ignore));
            e.error = r
                .error
                .as_deref()
                .filter(|s| !s.trim().is_empty())
                .map(|s| clean(s, MAX_ERROR, root, ignore));
            if let Some(c) = &r.command {
                if outcome::is_test_command(c) {
                    let failed = r.outcome == ToolOutcome::Failure;
                    e.test = Some(if failed { "fail" } else { "pass" }.into());
                }
            }
            if e.files.is_empty() && e.command.is_none() && e.error.is_none() {
                return None; // web searches, todo lists and the like
            }
        }
        EventKind::SessionEnd { reason } => {
            e.event = "session_end".into();
            e.reason = reason.clone();
        }
    }
    Some(e)
}

/// Redact, hide ignored paths, and clip.
fn clean(text: &str, max: usize, root: &Path, ignore: &Gitignore) -> String {
    let text = hide_ignored(text, root, ignore);
    episode::clip(&redact::redact(&text), max)
}

/// Replace words that name an ignored path (`cat secrets/prod.env`).
fn hide_ignored(text: &str, root: &Path, ignore: &Gitignore) -> String {
    if ignore.is_empty() {
        return text.to_string();
    }
    let mut changed = false;
    let words: Vec<String> = text
        .split(' ')
        .map(|w| {
            let bare = w.trim_matches(|c: char| {
                matches!(c, '"' | '\'' | '`' | ',' | ';' | ':' | '(' | ')')
            });
            let looks_like_path = bare.contains('/') || bare.contains('.');
            if looks_like_path && !bare.is_empty() {
                if let Some(rel) = relative(root, root, bare) {
                    if is_ignored(ignore, &rel) {
                        changed = true;
                        return w.replace(bare, "[ignored path]");
                    }
                }
            }
            w.to_string()
        })
        .collect();
    if changed {
        words.join(" ")
    } else {
        text.to_string()
    }
}

fn is_ignored(ignore: &Gitignore, rel: &str) -> bool {
    ignore.matched_path_or_any_parents(rel, false).is_ignore()
}

/// Repo-relative, forward-slash form of `path`, or `None` outside the repo.
pub fn relative(root: &Path, cwd: &Path, path: &str) -> Option<String> {
    let p = Path::new(path);
    let abs = if p.is_absolute() {
        p.to_path_buf()
    } else {
        cwd.join(p)
    };
    let abs = normalize(&abs);
    let rel = repo_rel(root, &abs).or_else(|| {
        let canon_root = root.canonicalize().ok()?;
        let canon = abs.canonicalize().unwrap_or(abs.clone());
        repo_rel(&canon_root, &canon)
    })?;
    if rel.is_empty() || rel == ".git" || rel.starts_with(".git/") {
        return None;
    }
    Some(rel)
}

/// Resolve `.` and `..` without touching the file system.
fn normalize(p: &Path) -> std::path::PathBuf {
    use std::path::Component;
    let mut out = std::path::PathBuf::new();
    for c in p.components() {
        match c {
            Component::CurDir => {}
            Component::ParentDir => {
                out.pop();
            }
            c => out.push(c),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use ignore::gitignore::GitignoreBuilder;
    use myrn_model::event::ToolResult;

    fn ignore(root: &Path) -> Gitignore {
        let mut b = GitignoreBuilder::new(root);
        b.add_line(None, "secrets/").unwrap();
        b.build().unwrap()
    }

    fn tool(outcome: ToolOutcome, files: &[&str], command: Option<&str>) -> Event {
        Event {
            session_id: "s".into(),
            cwd: Some("/r".into()),
            kind: EventKind::ToolResult(ToolResult {
                tool: "Bash".into(),
                outcome,
                files: files.iter().map(|s| s.to_string()).collect(),
                command: command.map(Into::into),
                error: None,
            }),
        }
    }

    #[test]
    fn paths_are_relative_and_private_paths_are_dropped() {
        let root = Path::new("/r");
        let ig = ignore(root);
        let e = to_entry(
            "claude",
            &tool(ToolOutcome::FileEdit, &["/r/src/a.rs", "src/./b.rs"], None),
            root,
            &ig,
            1,
        )
        .unwrap();
        assert_eq!(e.files, ["src/a.rs", "src/b.rs"]);
        assert_eq!(e.outcome.as_deref(), Some("file_edit"));
        // Only ignored or outside paths: nothing recorded.
        assert!(to_entry(
            "claude",
            &tool(ToolOutcome::Other, &["/r/secrets/prod.env"], None),
            root,
            &ig,
            1
        )
        .is_none());
        assert!(to_entry(
            "claude",
            &tool(ToolOutcome::Other, &["/etc/passwd"], None),
            root,
            &ig,
            1
        )
        .is_none());
        // Mixed: the ignored path is dropped, the rest kept.
        let e = to_entry(
            "claude",
            &tool(ToolOutcome::Other, &["/r/secrets/a", "/r/src/a.rs"], None),
            root,
            &ig,
            1,
        )
        .unwrap();
        assert_eq!(e.files, ["src/a.rs"]);
    }

    #[test]
    fn commands_are_redacted_and_hide_ignored_paths() {
        let root = Path::new("/r");
        let ig = ignore(root);
        let e = to_entry(
            "claude",
            &tool(
                ToolOutcome::Other,
                &[],
                Some("cat secrets/prod.env && GITHUB_TOKEN=abc123xyz gh pr list"),
            ),
            root,
            &ig,
            1,
        )
        .unwrap();
        assert_eq!(
            e.command.as_deref(),
            Some("cat [ignored path] && GITHUB_TOKEN=[REDACTED:secret] gh pr list")
        );
    }

    #[test]
    fn test_runs_get_an_outcome() {
        let root = Path::new("/r");
        let ig = ignore(root);
        let e = to_entry(
            "claude",
            &tool(ToolOutcome::Failure, &[], Some("cargo test")),
            root,
            &ig,
            1,
        )
        .unwrap();
        assert_eq!(e.test.as_deref(), Some("fail"));
        let e = to_entry(
            "claude",
            &tool(ToolOutcome::Other, &[], Some("cargo test")),
            root,
            &ig,
            1,
        )
        .unwrap();
        assert_eq!(e.test.as_deref(), Some("pass"));
        let e = to_entry(
            "claude",
            &tool(ToolOutcome::Other, &[], Some("ls")),
            root,
            &ig,
            1,
        )
        .unwrap();
        assert_eq!(e.test, None);
        // Tools with nothing to record.
        assert!(to_entry("claude", &tool(ToolOutcome::Other, &[], None), root, &ig, 1).is_none());
    }
}
