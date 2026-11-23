---
id: adapter-mode
type: summary
status: unverified
title: adapter::Mode
source: myrn summarize
about: sym:crates/myrn-adapters/src/adapter.rs#adapter::Mode
anchors:
- sym:crates/myrn-adapters/src/adapter.rs#adapter::Mode@39ea9d826efafc50
map_version: '2'
---
# adapter::Mode

Selects full hooks plus MCP setup, or pull-only setup, which adds an MCP server and an instruction-file line but no hooks and no recording.

## Behaviour

- Hooks -> lifecycle hooks and MCP server installed
- PullOnly -> MCP server and instruction-file line, no hooks, nothing recorded

## Terms

install mode, pull only, hook mode, no-hooks mode, recording

## Connections

- **Location:** `crates/myrn-adapters/src/adapter.rs:20-25`
- **Signature:** `pub enum Mode`
- **In:** [[file-crates-myrn-adapters-src-adapter-rs]]
