---
id: ctx-engine
type: summary
status: unverified
title: Ctx::engine
source: myrn summarize
about: sym:crates/myrn/src/cmd/mod.rs#cmd::Ctx::engine
anchors:
- sym:crates/myrn/src/cmd/mod.rs#cmd::Ctx::engine@945e7acd19675fd6
---
# Ctx::engine

Opens an Engine for the current working directory (using data_dir if set), then calls refresh() to bring the index up to date, so every call pays that refresh cost. Errors from current_dir, open, or refresh propagate to the caller.

## Connections

- **Location:** `crates/myrn/src/cmd/mod.rs:39-44`
- **Signature:** `pub fn engine(&self) -> Result<Engine>`
- **In:** [[file-crates-myrn-src-cmd-mod-rs]]
- **Calls:** [[engine-open]], [[engine-refresh]]
- **Called by:** [[debug-review]], [[log-run]], [[mcp-run]], [[query-explain]], [[query-impact]], [[query-history]], [[query-context]], [[query-brief]], [[show-run]], [[status-run]], [[summarize-run]]
