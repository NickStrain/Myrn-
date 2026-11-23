---
id: codex-install
type: summary
status: unverified
title: Codex::install
source: myrn summarize
about: sym:crates/myrn-adapters/src/codex.rs#codex::Codex::install
anchors:
- sym:crates/myrn-adapters/src/codex.rs#codex::Codex::install@c2a8ec182a3c7f45
map_version: '2'
---
# Codex::install

In Hooks mode, writes the five hook events to hooks.json. In PullOnly mode, removes myrn hooks and adds the AGENTS.md block. It always writes the MCP block and returns the changes plus a trust note.

## Behaviour

- Hooks mode -> every EVENTS entry is added with command_line and its timeout
- PullOnly mode -> myrn hooks removed only if any existed; AGENTS.md path goes into created when pull_only::add returns true
- always calls write_mcp; the manual note says to trust the project and not commit .codex files

## Terms

setup, wire up, hook install, pull only mode, trust note

## Connections

- **Location:** `crates/myrn-adapters/src/codex.rs:174-229`
- **Signature:** `fn install(&self, ctx: &InstallCtx, mode: Mode) -> Result<Installed>`
- **In:** [[file-crates-myrn-adapters-src-codex-rs]]
- **Calls:** [[adapter-rel]], [[adapter-read-json]], [[adapter-write-json]], [[hooks-json-remove]], [[hooks-json-add]], [[codex-hooks-path]], [[codex-config-path]], [[codex-instructions-path]], [[codex-command-line]], [[codex-write-mcp]], [[pull-only-add]]
