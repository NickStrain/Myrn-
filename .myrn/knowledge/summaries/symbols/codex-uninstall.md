---
id: codex-uninstall
type: summary
status: unverified
title: Codex::uninstall
source: myrn summarize
about: sym:crates/myrn-adapters/src/codex.rs#codex::Codex::uninstall
anchors:
- sym:crates/myrn-adapters/src/codex.rs#codex::Codex::uninstall@48bcd4928e85582c
---
# Codex::uninstall

Reverses install. It removes the AGENTS.md instruction block, removes myrn hook entries (deleting hooks.json if it becomes empty), strips the managed MCP block (deleting config.toml if nothing remains), and removes .codex/ if it is empty. It returns the list of changes made.

## Connections

- **Location:** `crates/myrn-adapters/src/codex.rs:231-274`
- **Signature:** `fn uninstall(&self, ctx: &InstallCtx, created: &[String]) -> Result<Vec<Change>>`
- **In:** [[file-crates-myrn-adapters-src-codex-rs]]
- **Calls:** [[adapter-rel]], [[adapter-read-json]], [[adapter-write-json]], [[hooks-json-remove]], [[codex-codex-dir]], [[codex-hooks-path]], [[codex-config-path]], [[codex-instructions-path]], [[codex-strip-mcp]], [[pull-only-remove]]
