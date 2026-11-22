---
id: codex-install
type: summary
status: unverified
title: Codex::install
source: myrn summarize
about: sym:crates/myrn-adapters/src/codex.rs#codex::Codex::install
anchors:
- sym:crates/myrn-adapters/src/codex.rs#codex::Codex::install@c2a8ec182a3c7f45
---
# Codex::install

Removes existing myrn hook entries. In Hooks mode it adds one command hook per EVENTS entry to hooks.json, and in PullOnly mode it adds the AGENTS.md instruction block. It always writes the MCP block, and it returns the touched files and a manual note telling the user to trust the project in Codex. It fails if config.toml already defines a foreign [mcp_servers.myrn] table.

## Connections

- **Location:** `crates/myrn-adapters/src/codex.rs:174-229`
- **Signature:** `fn install(&self, ctx: &InstallCtx, mode: Mode) -> Result<Installed>`
- **In:** [[file-crates-myrn-adapters-src-codex-rs]]
- **Calls:** [[adapter-rel]], [[adapter-read-json]], [[adapter-write-json]], [[hooks-json-remove]], [[hooks-json-add]], [[codex-hooks-path]], [[codex-config-path]], [[codex-instructions-path]], [[codex-command-line]], [[codex-write-mcp]], [[pull-only-add]]
