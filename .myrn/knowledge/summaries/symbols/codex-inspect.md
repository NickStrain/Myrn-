---
id: codex-inspect
type: summary
status: unverified
title: Codex::inspect
source: myrn summarize
about: sym:crates/myrn-adapters/src/codex.rs#codex::Codex::inspect
anchors:
- sym:crates/myrn-adapters/src/codex.rs#codex::Codex::inspect@ab7818de7da2f20f
map_version: '2'
---
# Codex::inspect

Returns a Finding for each check: each EVENTS hook in Hooks mode, or the AGENTS.md block in PullOnly mode. It also checks that the MCP command in config.toml matches the current binary.

## Behaviour

- hook entry absent -> ok false, 'missing'
- hook command differs from command_line for this binary -> ok false, 'out of date'
- MCP command equals the binary path string -> ok true; unparseable TOML -> MCP reported missing

## Terms

health check, verify install, doctor, drift check, status

## Connections

- **Location:** `crates/myrn-adapters/src/codex.rs:276-332`
- **Signature:** `fn inspect(&self, ctx: &InstallCtx, mode: Mode) -> Vec<Finding>`
- **In:** [[file-crates-myrn-adapters-src-codex-rs]]
- **Calls:** [[adapter-rel]], [[adapter-read-json]], [[hooks-json-find]], [[codex-hooks-path]], [[codex-config-path]], [[codex-instructions-path]], [[codex-command-line]], [[pull-only-has-block]]
