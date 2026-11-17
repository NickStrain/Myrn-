//! Claude Code adapter.
//!
//! Hooks go in `.claude/settings.local.json` (per developer, not committed),
//! each one calling the myrn binary directly through `args`, never a shell.
//! The MCP server is registered with `claude mcp add --scope local`.

use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::Result;
use myrn_model::event::{Event, EventKind, ToolOutcome, ToolResult};
use serde_json::{json, Value};

use crate::adapter::{clip, hooks_json, read_json, rel, str_field, write_json};
use crate::{detect, pull_only, Adapter, Change, Finding, InstallCtx, Installed, Mode};

pub struct ClaudeCode;

/// Hook events and their timeouts in seconds. SessionEnd hooks share a 1.5 s budget.
pub const EVENTS: [(&str, u64); 6] = [
    ("SessionStart", 30),
    ("UserPromptSubmit", 10),
    ("PostToolUse", 10),
    ("PostToolUseFailure", 10),
    ("Stop", 10),
    ("SessionEnd", 2),
];

const EDIT_TOOLS: [&str; 4] = ["Edit", "Write", "MultiEdit", "NotebookEdit"];

fn settings_path(root: &Path) -> PathBuf {
    root.join(".claude").join("settings.local.json")
}

fn instructions_path(root: &Path) -> PathBuf {
    root.join("CLAUDE.md")
}

fn is_ours(h: &Value) -> bool {
    let args: Vec<&str> = h
        .get("args")
        .and_then(Value::as_array)
        .map(|a| a.iter().filter_map(Value::as_str).collect())
        .unwrap_or_default();
    args.len() >= 2 && args[0] == "hook" && args[1] == "claude"
}

fn handler(binary: &Path, event: &str, timeout: u64) -> Value {
    json!({
        "type": "command",
        "command": binary.to_string_lossy(),
        "args": ["hook", "claude", event],
        "timeout": timeout,
    })
}

fn manual_mcp(binary: &Path) -> String {
    format!(
        "claude mcp add --scope local myrn -- \"{}\" mcp",
        binary.display()
    )
}

/// Run the `claude` CLI in the repo; `None` if it is not installed or not allowed.
fn claude_cli(ctx: &InstallCtx, args: &[&str]) -> Option<std::process::Output> {
    if !ctx.run_agent_cli {
        return None;
    }
    let bin = detect::which("claude")?;
    Command::new(bin)
        .args(args)
        .current_dir(&ctx.repo_root)
        .output()
        .ok()
}

impl Adapter for ClaudeCode {
    fn name(&self) -> &'static str {
        "claude"
    }

    fn display_name(&self) -> &'static str {
        "Claude Code"
    }

    fn detect(&self) -> Option<String> {
        if let Some(p) = detect::which("claude") {
            return Some(format!("`claude` at {}", p.display()));
        }
        let dir = detect::home()?.join(".claude");
        dir.is_dir().then(|| format!("{} exists", dir.display()))
    }

    fn install(&self, ctx: &InstallCtx, mode: Mode) -> Result<Installed> {
        let root = &ctx.repo_root;
        let mut out = Installed::default();
        let settings = settings_path(root);
        let mut doc = read_json(&settings)?;
        let removed = hooks_json::remove(&mut doc, &is_ours);
        if mode == Mode::Hooks {
            for (event, timeout) in EVENTS {
                hooks_json::add(&mut doc, event, handler(&ctx.binary, event, timeout));
            }
            write_json(&settings, &doc)?;
            out.written.push(rel(root, &settings));
            out.changes.push(Change {
                path: rel(root, &settings),
                what: format!(
                    "hooks for {} -> `{} hook claude <event>`",
                    EVENTS.map(|e| e.0).join(", "),
                    ctx.binary.display()
                ),
            });
        } else {
            if removed > 0 {
                write_json(&settings, &doc)?;
                out.changes.push(Change {
                    path: rel(root, &settings),
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
        let _ = claude_cli(ctx, &["mcp", "remove", "--scope", "local", "myrn"]);
        let bin = ctx.binary.to_string_lossy().to_string();
        match claude_cli(
            ctx,
            &["mcp", "add", "--scope", "local", "myrn", "--", &bin, "mcp"],
        ) {
            Some(o) if o.status.success() => out.changes.push(Change {
                path: "~/.claude.json (this project)".into(),
                what: format!("MCP server `myrn` -> `{bin} mcp`"),
            }),
            Some(o) => out.manual.push(format!(
                "`claude mcp add` failed ({}); run: {}",
                String::from_utf8_lossy(&o.stderr).trim(),
                manual_mcp(&ctx.binary)
            )),
            None => out.manual.push(format!(
                "register the MCP server: {}",
                manual_mcp(&ctx.binary)
            )),
        }
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
        let settings = settings_path(root);
        if settings.exists() {
            let mut doc = read_json(&settings)?;
            if hooks_json::remove(&mut doc, &is_ours) > 0 {
                write_json(&settings, &doc)?;
                changes.push(Change {
                    path: rel(root, &settings),
                    what: "removed myrn hooks".into(),
                });
            }
        }
        if let Some(o) = claude_cli(ctx, &["mcp", "remove", "--scope", "local", "myrn"]) {
            if o.status.success() {
                changes.push(Change {
                    path: "~/.claude.json (this project)".into(),
                    what: "removed MCP server `myrn`".into(),
                });
            }
        }
        Ok(changes)
    }

    fn inspect(&self, ctx: &InstallCtx, mode: Mode) -> Vec<Finding> {
        let root = &ctx.repo_root;
        let mut out = Vec::new();
        let settings = settings_path(root);
        let doc = read_json(&settings).unwrap_or_else(|_| json!({}));
        let want = ctx.binary.to_string_lossy().to_string();
        if mode == Mode::Hooks {
            for (event, _) in EVENTS {
                let found = hooks_json::find(&doc, event, &is_ours);
                let finding = match found.first() {
                    None => Finding {
                        ok: false,
                        what: format!("{event} hook missing from {}", rel(root, &settings)),
                    },
                    Some(h) if h.get("command").and_then(Value::as_str) != Some(want.as_str()) => {
                        Finding {
                            ok: false,
                            what: format!(
                            "{event} hook calls {} but myrn is at {want}; run `myrn connect` again",
                            h.get("command").and_then(Value::as_str).unwrap_or("?")
                        ),
                        }
                    }
                    Some(_) if found.len() > 1 => Finding {
                        ok: false,
                        what: format!("{event} hook installed {} times", found.len()),
                    },
                    Some(_) => Finding {
                        ok: true,
                        what: format!("{event} hook -> {want}"),
                    },
                };
                out.push(finding);
            }
        } else {
            let p = instructions_path(root);
            out.push(if pull_only::has_block(&p) {
                Finding {
                    ok: true,
                    what: format!("instruction block in {}", rel(root, &p)),
                }
            } else {
                Finding {
                    ok: false,
                    what: format!("instruction block missing from {}", rel(root, &p)),
                }
            });
        }
        match claude_cli(ctx, &["mcp", "get", "myrn"]) {
            Some(o) if o.status.success() => out.push(Finding {
                ok: true,
                what: "MCP server `myrn` registered".into(),
            }),
            Some(_) => out.push(Finding {
                ok: false,
                what: format!(
                    "MCP server `myrn` not registered; run: {}",
                    manual_mcp(&ctx.binary)
                ),
            }),
            None => out.push(Finding {
                ok: true,
                what: "MCP registration not checked (`claude` not on PATH)".into(),
            }),
        }
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
            "PostToolUse" | "PostToolUseFailure" => {
                let tool = str_field(p, "tool_name").unwrap_or_default();
                let input = p.get("tool_input").cloned().unwrap_or(Value::Null);
                let files: Vec<String> = ["file_path", "notebook_path"]
                    .iter()
                    .filter_map(|k| str_field(&input, k))
                    .collect();
                let outcome = if name == "PostToolUseFailure" {
                    ToolOutcome::Failure
                } else if EDIT_TOOLS.contains(&tool.as_str()) {
                    ToolOutcome::FileEdit
                } else {
                    ToolOutcome::Other
                };
                EventKind::ToolResult(ToolResult {
                    command: str_field(&input, "command"),
                    error: str_field(p, "error"),
                    tool,
                    outcome,
                    files,
                })
            }
            "Stop" => EventKind::SessionEnd {
                reason: Some("stop".into()),
                continued: p
                    .get("stop_hook_active")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            },
            "SessionEnd" => EventKind::SessionEnd {
                reason: str_field(p, "reason"),
                continued: false,
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
        if context.trim().is_empty() {
            return String::new();
        }
        match event {
            // Keep the agent going for one more step with this instruction.
            // The agent marks the next Stop with `stop_hook_active`, so it is asked once.
            "Stop" => json!({ "decision": "block", "reason": clip(context) }).to_string(),
            "SessionStart" | "UserPromptSubmit" | "PostToolUse" | "PostToolUseFailure" => {
                json!({ "hookSpecificOutput": { "hookEventName": event, "additionalContext": clip(context) } }).to_string()
            }
            _ => String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx(root: &Path) -> InstallCtx {
        InstallCtx {
            repo_root: root.to_path_buf(),
            binary: PathBuf::from("/opt/myrn/bin/myrn"),
            run_agent_cli: false,
        }
    }

    #[test]
    fn install_is_idempotent_and_keeps_other_hooks() {
        let dir = tempfile::tempdir().unwrap();
        let settings = settings_path(dir.path());
        write_json(
            &settings,
            &json!({"permissions": {"allow": ["Bash(ls)"]},
                    "hooks": {"PostToolUse": [{"matcher": "Edit", "hooks": [{"type": "command", "command": "lint"}]}]}}),
        )
        .unwrap();
        let a = ClaudeCode;
        a.install(&ctx(dir.path()), Mode::Hooks).unwrap();
        a.install(&ctx(dir.path()), Mode::Hooks).unwrap();
        let doc = read_json(&settings).unwrap();
        assert_eq!(doc["permissions"]["allow"][0], "Bash(ls)");
        for (e, _) in EVENTS {
            assert_eq!(hooks_json::find(&doc, e, &is_ours).len(), 1, "{e}");
        }
        let post = doc["hooks"]["PostToolUse"].as_array().unwrap();
        assert!(post.iter().any(|g| g["hooks"][0]["command"] == "lint"));
        let h = &hooks_json::find(&doc, "SessionStart", &is_ours)[0];
        assert_eq!(h["command"], "/opt/myrn/bin/myrn");
        assert_eq!(h["args"], json!(["hook", "claude", "SessionStart"]));
        assert!(a
            .inspect(&ctx(dir.path()), Mode::Hooks)
            .iter()
            .all(|f| f.ok));

        // Drift: binary moved.
        let moved = InstallCtx {
            binary: PathBuf::from("/new/myrn"),
            ..ctx(dir.path())
        };
        assert!(a.inspect(&moved, Mode::Hooks).iter().any(|f| !f.ok));

        a.uninstall(&ctx(dir.path()), &[]).unwrap();
        let doc = read_json(&settings).unwrap();
        assert_eq!(
            doc["hooks"]["PostToolUse"][0]["hooks"][0]["command"],
            "lint"
        );
        assert!(doc["hooks"].get("SessionStart").is_none());
    }

    #[test]
    fn pull_only_writes_instruction_block_not_hooks() {
        let dir = tempfile::tempdir().unwrap();
        let a = ClaudeCode;
        let r = a.install(&ctx(dir.path()), Mode::PullOnly).unwrap();
        assert_eq!(r.created, ["CLAUDE.md"]);
        assert!(pull_only::has_block(&dir.path().join("CLAUDE.md")));
        assert!(!settings_path(dir.path()).exists());
        assert!(r
            .manual
            .iter()
            .any(|m| m.contains("claude mcp add --scope local myrn")));
        a.uninstall(&ctx(dir.path()), &r.created).unwrap();
        assert!(!dir.path().join("CLAUDE.md").exists());
    }

    fn payload(name: &str) -> Value {
        let p = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/fixtures/hook-payloads/claude-code")
            .join(name);
        serde_json::from_str(&std::fs::read_to_string(p).unwrap()).unwrap()
    }

    #[test]
    fn translates_recorded_payloads() {
        let a = ClaudeCode;
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
        assert_eq!(e.session_id, "abc123");
        let e = a
            .translate("UserPromptSubmit", &payload("user_prompt_submit.json"))
            .unwrap()
            .unwrap();
        assert_eq!(
            e.kind,
            EventKind::PromptSubmitted {
                prompt: "Fix the 401 when a session expires".into()
            }
        );
        let e = a
            .translate("PostToolUse", &payload("post_tool_use_edit.json"))
            .unwrap()
            .unwrap();
        match e.kind {
            EventKind::ToolResult(t) => {
                assert_eq!(t.outcome, ToolOutcome::FileEdit);
                assert_eq!(t.files, ["/home/dev/proj/src/auth/session.rs"]);
            }
            k => panic!("{k:?}"),
        }
        let e = a
            .translate("PostToolUse", &payload("post_tool_use_bash.json"))
            .unwrap()
            .unwrap();
        match e.kind {
            EventKind::ToolResult(t) => {
                assert_eq!(t.outcome, ToolOutcome::Other);
                assert_eq!(t.command.as_deref(), Some("cargo test auth"));
            }
            k => panic!("{k:?}"),
        }
        let e = a
            .translate("PostToolUseFailure", &payload("post_tool_use_failure.json"))
            .unwrap()
            .unwrap();
        match e.kind {
            EventKind::ToolResult(t) => {
                assert_eq!(t.outcome, ToolOutcome::Failure);
                assert!(t.error.unwrap().contains("test result: FAILED"));
            }
            k => panic!("{k:?}"),
        }
        let e = a
            .translate("SessionEnd", &payload("session_end.json"))
            .unwrap()
            .unwrap();
        assert_eq!(
            e.kind,
            EventKind::SessionEnd {
                reason: Some("prompt_input_exit".into()),
                continued: false,
            }
        );
        assert!(a
            .translate("Notification", &json!({"hook_event_name": "Notification"}))
            .unwrap()
            .is_none());
    }

    #[test]
    fn formats_additional_context() {
        let a = ClaudeCode;
        let out: Value = serde_json::from_str(&a.format("UserPromptSubmit", "ctx")).unwrap();
        assert_eq!(
            out["hookSpecificOutput"]["hookEventName"],
            "UserPromptSubmit"
        );
        assert_eq!(out["hookSpecificOutput"]["additionalContext"], "ctx");
        assert_eq!(a.format("UserPromptSubmit", ""), "");
        // Recall on a failed tool goes alongside the error.
        let out: Value = serde_json::from_str(&a.format("PostToolUseFailure", "bug")).unwrap();
        assert_eq!(
            out["hookSpecificOutput"]["hookEventName"],
            "PostToolUseFailure"
        );
        assert_eq!(out["hookSpecificOutput"]["additionalContext"], "bug");
        // The record request keeps the agent going once.
        let out: Value = serde_json::from_str(&a.format("Stop", "record this")).unwrap();
        assert_eq!(out, json!({"decision": "block", "reason": "record this"}));
        assert_eq!(a.format("Stop", ""), "");
        assert_eq!(a.format("SessionEnd", "ctx"), "");
        let big = "x".repeat(20_000);
        let out: Value = serde_json::from_str(&a.format("SessionStart", &big)).unwrap();
        assert!(
            out["hookSpecificOutput"]["additionalContext"]
                .as_str()
                .unwrap()
                .len()
                <= crate::adapter::MAX_CONTEXT_CHARS
        );
    }
}
