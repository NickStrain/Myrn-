---
id: codex-uninstall
type: summary
status: unverified
title: Codex::uninstall
source: myrn summarize
about: sym:crates/myrn-adapters/src/codex.rs#codex::Codex::uninstall
anchors:
- sym:crates/myrn-adapters/src/codex.rs#codex::Codex::uninstall@48bcd4928e85582c
map_version: '2'
---
# Codex::uninstall

Removes the AGENTS.md block, myrn hook entries and the MCP block. It deletes files left empty and removes .codex/ if it is empty.

## Behaviour

- hooks.json with only myrn hooks left -> the file is deleted
- config.toml with only the myrn block left -> the file is deleted
- other user content is kept; the .codex directory removal error is ignored

## Terms

teardown, undo install, clean up, remove hooks, disconnect

## Connections

- **Location:** `crates/myrn-adapters/src/codex.rs:231-274`
- **Signature:** `fn uninstall(&self, ctx: &InstallCtx, created: &[String]) -> Result<Vec<Change>>`
- **In:** [[file-crates-myrn-adapters-src-codex-rs]]
- **Calls:** [[adapter-rel]], [[adapter-read-json]], [[adapter-write-json]], [[hooks-json-remove]], [[codex-codex-dir]], [[codex-hooks-path]], [[codex-config-path]], [[codex-instructions-path]], [[codex-strip-mcp]], [[pull-only-remove]]
