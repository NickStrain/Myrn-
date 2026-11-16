//! Codex adapter.
//!
//! Hooks go in `<repo>/.codex/hooks.json` and the MCP server in
//! `<repo>/.codex/config.toml`. Codex loads both only for trusted projects.
//! Codex hook handlers have no argument list, so the command is a single
//! quoted line (`"/path/myrn" hook codex <event>`) that runs the binary
//! directly under both POSIX shells and cmd.exe; no script is involved.

use std::path::{Path, PathBuf};

use anyhow::Result;
use myrn_model::event::{Event, EventKind, ToolOutcome, ToolResult};
use serde_json::{json, Value};

use crate::adapter::{clip, hooks_json, read_json, rel, str_field, write_json};
use crate::{detect, pull_only, Adapter, Change, Finding, InstallCtx, Installed, Mode};

pub struct Codex;

/// Hook events and timeouts in seconds (SessionEnd allows at most 3).
pub const EVENTS: [(&str, u64); 5] = [
    ("SessionStart", 30),
    ("UserPromptSubmit", 10),
    ("PostToolUse", 10),
    ("Stop", 10),
    ("SessionEnd", 3),
];

const MCP_START: &str =
    "# myrn:start (managed by `myrn connect`; edit with `myrn connect`/`myrn disconnect`)";
const MCP_END: &str = "# myrn:end";

fn codex_dir(root: &Path) -> PathBuf {
    root.join(".codex")
}
fn hooks_path(root: &Path) -> PathBuf {
    codex_dir(root).join("hooks.json")
}
fn config_path(root: &Path) -> PathBuf {
    codex_dir(root).join("config.toml")
}
fn instructions_path(root: &Path) -> PathBuf {
    root.join("AGENTS.md")
}

fn command_line(binary: &Path, event: &str) -> String {
    format!("\"{}\" hook codex {event}", binary.display())
}

fn is_ours(h: &Value) -> bool {
    h.get("command")
        .and_then(Value::as_str)
        .is_some_and(|c| c.contains(" hook codex "))
}

fn mcp_block(binary: &Path) -> String {
    let cmd = toml::Value::String(binary.to_string_lossy().into_owned()).to_string();
    format!("{MCP_START}\n[mcp_servers.myrn]\ncommand = {cmd}\nargs = [\"mcp\"]\n{MCP_END}\n")
}

/// Remove our block from a TOML text.
fn strip_mcp(s: &str) -> String {
    let (Some(a), Some(b)) = (s.find(MCP_START), s.find(MCP_END)) else {
        return s.to_string();
    };
    if b < a {
        return s.to_string();
    }
    let mut end = b + MCP_END.len();
    if s[end..].starts_with('\n') {
        end += 1;
    }
    let before = s[..a].trim_end_matches('\n');
    let after = &s[end..];
    if before.is_empty() {
        after.to_string()
    } else {
        format!("{before}\n{after}")
    }
}

fn write_mcp(root: &Path, binary: &Path) -> Result<()> {
    let p = config_path(root);
    let existing = std::fs::read_to_string(&p).unwrap_or_default();
    let mut body = strip_mcp(&existing);
    if toml::from_str::<toml::Table>(&body)
        .ok()
        .and_then(|t| t.get("mcp_servers").and_then(|m| m.get("myrn")).cloned())
        .is_some()
    {
        anyhow::bail!(
            "{} already defines [mcp_servers.myrn] outside myrn's block; remove it first",
            p.display()
        );
    }
    if !body.is_empty() && !body.ends_with('\n') {
        body.push('\n');
    }
    if !body.is_empty() {
        body.push('\n');
    }
    std::fs::create_dir_all(codex_dir(root))?;
    std::fs::write(&p, body + &mcp_block(binary))?;
    Ok(())
}

/// Files named in an apply_patch body.
pub fn patch_files(patch: &str) -> Vec<String> {
    const MARKERS: [&str; 4] = [
        "*** Add File: ",
        "*** Update File: ",
        "*** Delete File: ",
        "*** Move to: ",
    ];
    let mut out: Vec<String> = Vec::new();
    for line in patch.lines() {
        for m in MARKERS {
            if let Some(p) = line.strip_prefix(m) {
                let p = p.trim().to_string();
                if !out.contains(&p) {
                    out.push(p);
                }
            }
        }
    }
    out
}

/// Codex does not document a failure event; read what the tool response carries.
fn failure_text(resp: &Value) -> Option<String> {
    for key in ["exit_code", "exitCode"] {
        if let Some(code) = resp.get(key).and_then(Value::as_i64) {
            if code != 0 {
                let stderr = str_field(resp, "stderr")
                    .or_else(|| str_field(resp, "output"))
                    .unwrap_or_default();
                return Some(format!("Exit code {code}\n{stderr}").trim_end().to_string());
            }
            return None;
        }
    }
    if let Some(e) = resp.get("error").filter(|e| !e.is_null()) {
        return Some(
            e.as_str()
                .map(str::to_string)
                .unwrap_or_else(|| e.to_string()),
        );
    }
    if resp.get("success").and_then(Value::as_bool) == Some(false) {
        return Some(resp.to_string());
    }
    None
}

impl Adapter for Codex {
    fn name(&self) -> &'static str {
        "codex"
    }

    fn display_name(&self) -> &'static str {
        "Codex"
    }

    fn detect(&self) -> Option<String> {
        if let Some(p) = detect::which("codex") {
            return Some(format!("`codex` at {}", p.display()));
        }
        let dir = std::env::var_os("CODEX_HOME")
            .map(PathBuf::from)
            .or_else(|| detect::home().map(|h| h.join(".codex")))?;
        dir.is_dir().then(|| format!("{} exists", dir.display()))
    }

    fn install(&self, ctx: &InstallCtx, mode: Mode) -> Result<Installed> {
        let root = &ctx.repo_root;
        let mut out = Installed::default();
        let hooks = hooks_path(root);
        let mut doc = read_json(&hooks)?;
        let removed = hooks_json::remove(&mut doc, &is_ours);
        if mode == Mode::Hooks {
            for (event, timeout) in EVENTS {
                hooks_json::add(
                    &mut doc,
                    event,
                    json!({ "type": "command", "command": command_line(&ctx.binary, event), "timeout": timeout }),
                );
            }
            write_json(&hooks, &doc)?;
            out.written.push(rel(root, &hooks));
            out.changes.push(Change {
                path: rel(root, &hooks),
                what: format!(
                    "hooks for {} -> `{}`",
                    EVENTS.map(|e| e.0).join(", "),
                    command_line(&ctx.binary, "<event>")
                ),
            });
        } else {
            if removed > 0 {
                write_json(&hooks, &doc)?;
                out.changes.push(Change {
                    path: rel(root, &hooks),
                    what: "removed myrn hooks".into(),
                });
            }
            let p = instructions_path(root);
            if pull_only::add(&p)? {
                out.created.push(rel(root, &p));
            }
            out.written.push(rel(root, &p));
            out.changes.push(Change {
                path: rel(root, &p),
                what: "line asking the agent to call `context` first".into(),
            });
        }
        write_mcp(root, &ctx.binary)?;
        out.written.push(rel(root, &config_path(root)));
        out.changes.push(Change {
            path: rel(root, &config_path(root)),
            what: format!("[mcp_servers.myrn] -> `{} mcp`", ctx.binary.display()),
        });
        out.manual.push(
            "Codex reads .codex/ only in trusted projects: open `codex` in this repository and trust it \
             (and review the hooks when Codex asks). .codex/hooks.json and .codex/config.toml contain this \
             machine's path to myrn; don't commit them."
                .into(),
        );
        Ok(out)
    }

    fn uninstall(&self, ctx: &InstallCtx, created: &[String]) -> Result<Vec<Change>> {
        let root = &ctx.repo_root;
        let mut changes = Vec::new();
        let p = instructions_path(root);
        if pull_only::remove(&p, created.contains(&rel(root, &p)))? {
            changes.push(Change {
                path: rel(root, &p),
                what: "removed myrn instruction block".into(),
            });
        }
        let hooks = hooks_path(root);
        if hooks.exists() {
            let mut doc = read_json(&hooks)?;
            if hooks_json::remove(&mut doc, &is_ours) > 0 {
                if doc.as_object().is_some_and(|o| o.is_empty()) {
                    std::fs::remove_file(&hooks)?;
                } else {
                    write_json(&hooks, &doc)?;
                }
                changes.push(Change {
                    path: rel(root, &hooks),
                    what: "removed myrn hooks".into(),
                });
            }
        }
        let cfg = config_path(root);
        if let Ok(s) = std::fs::read_to_string(&cfg) {
            if s.contains(MCP_START) {
                let rest = strip_mcp(&s);
                if rest.trim().is_empty() {
                    std::fs::remove_file(&cfg)?;
                } else {
                    std::fs::write(&cfg, rest)?;
                }
                changes.push(Change {
                    path: rel(root, &cfg),
                    what: "removed [mcp_servers.myrn]".into(),
                });
            }
        }
        // Leave .codex/ itself if anything else is in it.
        let _ = std::fs::remove_dir(codex_dir(root));
        Ok(changes)
    }

    fn inspect(&self, ctx: &InstallCtx, mode: Mode) -> Vec<Finding> {
        let root = &ctx.repo_root;
        let mut out = Vec::new();
        if mode == Mode::Hooks {
            let doc = read_json(&hooks_path(root)).unwrap_or_else(|_| json!({}));
            for (event, _) in EVENTS {
                let want = command_line(&ctx.binary, event);
                let found = hooks_json::find(&doc, event, &is_ours);
                out.push(match found.first() {
                    None => Finding {
                        ok: false,
                        what: format!("{event} hook missing from .codex/hooks.json"),
                    },
                    Some(h) if h.get("command").and_then(Value::as_str) != Some(want.as_str()) => {
                        Finding {
                            ok: false,
                            what: format!("{event} hook is out of date; run `myrn connect` again"),
                        }
                    }
                    Some(_) => Finding {
                        ok: true,
                        what: format!("{event} hook -> {want}"),
                    },
                });
            }
        } else {
            let p = instructions_path(root);
            out.push(Finding {
                ok: pull_only::has_block(&p),
                what: format!("instruction block in {}", rel(root, &p)),
            });
        }
        let cfg = std::fs::read_to_string(config_path(root)).unwrap_or_default();
        let parsed = toml::from_str::<toml::Table>(&cfg).ok();
        let cmd = parsed.as_ref().and_then(|t| {
            t.get("mcp_servers")?
                .get("myrn")?
                .get("command")?
                .as_str()
                .map(str::to_string)
        });
        out.push(match cmd {
            Some(c) if c == ctx.binary.to_string_lossy() => Finding {
                ok: true,
                what: "MCP server `myrn` in .codex/config.toml".into(),
            },
            Some(c) => Finding {
                ok: false,
                what: format!("MCP server `myrn` points at {c}; run `myrn connect` again"),
            },
            None => Finding {
                ok: false,
                what: "MCP server `myrn` missing from .codex/config.toml".into(),
            },
        });
        out
    }

    fn translate(&self, event: &str, p: &Value) -> Result<Option<Event>> {
        let name = str_field(p, "hook_event_name").unwrap_or_else(|| event.to_string());
        let session_id = str_field(p, "session_id").unwrap_or_default();
        let cwd = str_field(p, "cwd");
        let kind = match name.as_str() {
            "SessionStart" => EventKind::SessionStart {
                source: str_field(p, "source"),
            },
            "UserPromptSubmit" => EventKind::PromptSubmitted {
                prompt: str_field(p, "prompt").unwrap_or_default(),
            },
            "PostToolUse" => {
                let tool = str_field(p, "tool_name").unwrap_or_default();
                let input = p.get("tool_input").cloned().unwrap_or(Value::Null);
                let resp = p.get("tool_response").cloned().unwrap_or(Value::Null);
                let command = str_field(&input, "command");
                let is_patch = tool == "apply_patch";
                let files = if is_patch {
                    patch_files(command.as_deref().unwrap_or(""))
                } else {
                    ["file_path", "path"]
                        .iter()
                        .filter_map(|k| str_field(&input, k))
                        .collect()
                };
                let error = failure_text(&resp);
                let outcome = if error.is_some() {
                    ToolOutcome::Failure
                } else if is_patch || matches!(tool.as_str(), "Edit" | "Write") {
                    ToolOutcome::FileEdit
                } else {
                    ToolOutcome::Other
                };
                EventKind::ToolResult(ToolResult {
                    command: if is_patch { None } else { command },
                    tool,
                    outcome,
                    files,
                    error,
                })
            }
            "Stop" => EventKind::SessionEnd {
                reason: Some("stop".into()),
            },
            "SessionEnd" => EventKind::SessionEnd {
                reason: str_field(p, "reason"),
            },
            _ => return Ok(None),
        };
        Ok(Some(Event {
            session_id,
            cwd,
            kind,
        }))
    }

    fn format(&self, event: &str, context: &str) -> String {
        if context.trim().is_empty() || !matches!(event, "SessionStart" | "UserPromptSubmit") {
            return String::new();
        }
        json!({ "hookSpecificOutput": { "hookEventName": event, "additionalContext": clip(context) } }).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx(root: &Path) -> InstallCtx {
        InstallCtx {
            repo_root: root.to_path_buf(),
            binary: PathBuf::from("/opt/my rn/myrn"),
            run_agent_cli: false,
        }
    }

    #[test]
    fn install_preserves_config_and_uninstalls_cleanly() {
        let dir = tempfile::tempdir().unwrap();
        let cfg = config_path(dir.path());
        std::fs::create_dir_all(cfg.parent().unwrap()).unwrap();
        let original = "# my settings\nmodel = \"o5\"\n\n[mcp_servers.other]\ncommand = \"x\"\n";
        std::fs::write(&cfg, original).unwrap();
        let a = Codex;
        a.install(&ctx(dir.path()), Mode::Hooks).unwrap();
        a.install(&ctx(dir.path()), Mode::Hooks).unwrap();
        let text = std::fs::read_to_string(&cfg).unwrap();
        assert_eq!(text.matches(MCP_START).count(), 1);
        let t: toml::Table = toml::from_str(&text).unwrap();
        assert_eq!(
            t["mcp_servers"]["myrn"]["command"].as_str(),
            Some("/opt/my rn/myrn")
        );
        assert_eq!(t["mcp_servers"]["other"]["command"].as_str(), Some("x"));
        let doc = read_json(&hooks_path(dir.path())).unwrap();
        let h = &hooks_json::find(&doc, "UserPromptSubmit", &is_ours);
        assert_eq!(h.len(), 1);
        assert_eq!(
            h[0]["command"],
            "\"/opt/my rn/myrn\" hook codex UserPromptSubmit"
        );
        assert!(a
            .inspect(&ctx(dir.path()), Mode::Hooks)
            .iter()
            .all(|f| f.ok));

        a.uninstall(&ctx(dir.path()), &[]).unwrap();
        assert_eq!(std::fs::read_to_string(&cfg).unwrap(), original);
        assert!(!hooks_path(dir.path()).exists());
    }

    #[test]
    fn pull_only_uses_agents_md() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("AGENTS.md"), "# Agents\n").unwrap();
        let a = Codex;
        let r = a.install(&ctx(dir.path()), Mode::PullOnly).unwrap();
        assert!(r.created.is_empty());
        assert!(pull_only::has_block(&dir.path().join("AGENTS.md")));
        assert!(!hooks_path(dir.path()).exists());
        assert!(a
            .inspect(&ctx(dir.path()), Mode::PullOnly)
            .iter()
            .all(|f| f.ok));
    }

    fn payload(name: &str) -> Value {
        let p = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/fixtures/hook-payloads/codex")
            .join(name);
        serde_json::from_str(&std::fs::read_to_string(p).unwrap()).unwrap()
    }

    #[test]
    fn translates_recorded_payloads() {
        let a = Codex;
        let e = a
            .translate("SessionStart", &payload("session_start.json"))
            .unwrap()
            .unwrap();
        assert_eq!(
            e.kind,
            EventKind::SessionStart {
                source: Some("startup".into())
            }
        );
        let e = a
            .translate("UserPromptSubmit", &payload("user_prompt_submit.json"))
            .unwrap()
            .unwrap();
        assert_eq!(
            e.kind,
            EventKind::PromptSubmitted {
                prompt: "why does refresh return 401?".into()
            }
        );
        match a
            .translate("PostToolUse", &payload("post_tool_use_apply_patch.json"))
            .unwrap()
            .unwrap()
            .kind
        {
            EventKind::ToolResult(t) => {
                assert_eq!(t.outcome, ToolOutcome::FileEdit);
                assert_eq!(t.files, ["src/auth/session.rs", "src/auth/clock.rs"]);
                assert!(t.command.is_none());
            }
            k => panic!("{k:?}"),
        }
        match a
            .translate("PostToolUse", &payload("post_tool_use_bash_failed.json"))
            .unwrap()
            .unwrap()
            .kind
        {
            EventKind::ToolResult(t) => {
                assert_eq!(t.outcome, ToolOutcome::Failure);
                assert_eq!(t.command.as_deref(), Some("cargo test"));
                assert!(t.error.unwrap().starts_with("Exit code 101"));
            }
            k => panic!("{k:?}"),
        }
        let e = a.translate("Stop", &payload("stop.json")).unwrap().unwrap();
        assert_eq!(
            e.kind,
            EventKind::SessionEnd {
                reason: Some("stop".into())
            }
        );
    }

    #[test]
    fn patch_file_names() {
        let p = "*** Begin Patch\n*** Update File: a.rs\n@@\n-x\n+y\n*** Add File: b.rs\n+z\n*** End Patch";
        assert_eq!(patch_files(p), ["a.rs", "b.rs"]);
    }
}
