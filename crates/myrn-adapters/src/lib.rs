//! Adapters: all agent-specific code lives here. Each adapter does two jobs:
//! install (write hook config + register MCP) and translate (payload <-> events).
//!
//! Design doc: "Agent integration > Adapter contract".

pub mod adapter;
pub mod claude_code;
pub mod codex;
pub mod detect;
pub mod pull_only;
