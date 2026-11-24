//! The adapter contract.

use std::path::{Path, PathBuf};

use anyhow::Result;
use myrn_model::event::Event;
use serde_json::Value;

/// What an install needs to know.
#[derive(Debug, Clone)]
pub struct InstallCtx {
    pub repo_root: PathBuf,
    /// Absolute path of the `myrn` binary hooks should call.
    pub binary: PathBuf,
    /// Run the agent's own CLI (e.g. `claude mcp add`) when it is installed.
    pub run_agent_cli: bool,
    /// Register myrn's MCP server in hooks mode too (the default; `myrn
    /// connect --no-mcp` leaves it out). Pull-only mode always has it.
    pub mcp: bool,
}

impl InstallCtx {
    /// Whether this install registers the MCP server.
    pub fn wants_mcp(&self, mode: Mode) -> bool {
        mode == Mode::PullOnly || self.mcp
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// Lifecycle hooks plus the MCP server.
    Hooks,
    /// MCP server plus a line in the agent's instruction file; no hooks, nothing recorded.
    PullOnly,
}

impl Mode {
    pub fn as_str(self) -> &'static str {
        match self {
            Mode::Hooks => "hooks",
            Mode::PullOnly => "pull_only",
        }
    }
    pub fn parse(s: &str) -> Mode {
        if s == "pull_only" {
            Mode::PullOnly
        } else {
            Mode::Hooks
        }
    }
}

/// One change made to the developer's configuration, shown to them.
#[derive(Debug, Clone)]
pub struct Change {
    pub path: String,
    pub what: String,
}

#[derive(Debug, Clone, Default)]
pub struct Installed {
    pub changes: Vec<Change>,
    /// Files written or edited.
    pub written: Vec<String>,
    /// Instruction files created from nothing (deleted again on disconnect if left empty).
    pub created: Vec<String>,
    /// Things the developer still has to do by hand.
    pub manual: Vec<String>,
}

/// A doctor check result for one installed piece.
#[derive(Debug, Clone)]
pub struct Finding {
    pub ok: bool,
    pub what: String,
}

pub trait Adapter {
    /// Short id used on the command line and in hook commands: `claude`, `codex`.
    fn name(&self) -> &'static str;
    fn display_name(&self) -> &'static str;
    /// Why we think the agent is installed, or `None`.
    fn detect(&self) -> Option<String>;
    fn install(&self, ctx: &InstallCtx, mode: Mode) -> Result<Installed>;
    /// Remove everything `install` wrote. `created` lists instruction files myrn
    /// created; those are deleted if nothing but myrn's block was in them.
    fn uninstall(&self, ctx: &InstallCtx, created: &[String]) -> Result<Vec<Change>>;
    /// Compare what is installed with what `install` would write.
    fn inspect(&self, ctx: &InstallCtx, mode: Mode) -> Vec<Finding>;
    /// Translate a hook payload. `event` is the event name from the hook command line.
    /// `None` for events myrn does not use.
    fn translate(&self, event: &str, payload: &Value) -> Result<Option<Event>>;
    /// Hook stdout that hands `context` to the model for this agent event.
    fn format(&self, event: &str, context: &str) -> String;
}

/// Repo-relative path for display.
pub fn rel(root: &Path, p: &Path) -> String {
    p.strip_prefix(root)
        .unwrap_or(p)
        .to_string_lossy()
        .replace('\\', "/")
}

pub(crate) fn str_field(v: &Value, key: &str) -> Option<String> {
    v.get(key).and_then(Value::as_str).map(str::to_string)
}

/// Agents cap hook context; stay under the smallest cap (Codex ~2,500 tokens).
pub const MAX_CONTEXT_CHARS: usize = 9_000;

pub(crate) fn clip(s: &str) -> String {
    if s.chars().count() <= MAX_CONTEXT_CHARS {
        return s.to_string();
    }
    let mut out: String = s.chars().take(MAX_CONTEXT_CHARS - 20).collect();
    out.push_str("\n[truncated by myrn]");
    out
}

/// Read a JSON object file, or an empty object if missing.
pub(crate) fn read_json(path: &Path) -> Result<Value> {
    match std::fs::read_to_string(path) {
        Ok(s) if s.trim().is_empty() => Ok(serde_json::json!({})),
        Ok(s) => serde_json::from_str(&s)
            .map_err(|e| anyhow::anyhow!("{} is not valid JSON: {e}", path.display())),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(serde_json::json!({})),
        Err(e) => Err(e.into()),
    }
}

pub(crate) fn write_json(path: &Path, v: &Value) -> Result<()> {
    if let Some(d) = path.parent() {
        std::fs::create_dir_all(d)?;
    }
    std::fs::write(path, serde_json::to_string_pretty(v)? + "\n")?;
    Ok(())
}

/// Shared hooks-JSON editing for agents using the `{"hooks": {Event: [{matcher?, hooks: [handler]}]}}` layout.
pub(crate) mod hooks_json {
    use serde_json::{json, Value};

    /// Remove every handler `is_ours` matches; drop groups and events left empty.
    pub fn remove(doc: &mut Value, is_ours: &dyn Fn(&Value) -> bool) -> usize {
        let mut removed = 0;
        let Some(hooks) = doc.get_mut("hooks").and_then(Value::as_object_mut) else {
            return 0;
        };
        for groups in hooks.values_mut() {
            let Some(groups) = groups.as_array_mut() else {
                continue;
            };
            for g in groups.iter_mut() {
                if let Some(hs) = g.get_mut("hooks").and_then(Value::as_array_mut) {
                    let before = hs.len();
                    hs.retain(|h| !is_ours(h));
                    removed += before - hs.len();
                }
            }
            groups.retain(|g| {
                g.get("hooks")
                    .and_then(Value::as_array)
                    .is_none_or(|a| !a.is_empty())
            });
        }
        hooks.retain(|_, g| g.as_array().is_none_or(|a| !a.is_empty()));
        if hooks.is_empty() {
            if let Some(o) = doc.as_object_mut() {
                o.remove("hooks");
            }
        }
        removed
    }

    pub fn add(doc: &mut Value, event: &str, handler: Value) {
        if !doc.is_object() {
            *doc = json!({});
        }
        let hooks = doc
            .as_object_mut()
            .unwrap()
            .entry("hooks")
            .or_insert_with(|| json!({}));
        let groups = hooks
            .as_object_mut()
            .unwrap()
            .entry(event)
            .or_insert_with(|| json!([]));
        if let Some(a) = groups.as_array_mut() {
            a.push(json!({ "hooks": [handler] }));
        }
    }

    /// Our handlers for `event`.
    pub fn find<'a>(
        doc: &'a Value,
        event: &str,
        is_ours: &dyn Fn(&Value) -> bool,
    ) -> Vec<&'a Value> {
        doc.get("hooks")
            .and_then(|h| h.get(event))
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(|g| g.get("hooks").and_then(Value::as_array))
            .flatten()
            .filter(|h| is_ours(h))
            .collect()
    }
}
