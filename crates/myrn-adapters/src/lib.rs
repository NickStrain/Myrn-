//! Adapters: all agent-specific code lives here. Each adapter does two jobs:
//! install (write hook config and register the MCP server) and translate
//! (agent hook payload <-> normalized event).
//!
//! Design doc: "Agent integration > Adapter contract".

pub mod adapter;
pub mod claude_code;
pub mod codex;
pub mod detect;
pub mod pull_only;

pub use adapter::{Adapter, Change, Finding, InstallCtx, Installed, Mode};

/// Every known adapter.
pub fn all() -> Vec<Box<dyn Adapter>> {
    vec![Box::new(claude_code::ClaudeCode), Box::new(codex::Codex)]
}

pub fn by_name(name: &str) -> Option<Box<dyn Adapter>> {
    all().into_iter().find(|a| a.name() == name)
}
