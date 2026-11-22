---
id: codex-inspect
type: summary
status: unverified
title: Codex::inspect
source: myrn summarize
about: sym:crates/myrn-adapters/src/codex.rs#codex::Codex::inspect
anchors:
- sym:crates/myrn-adapters/src/codex.rs#codex::Codex::inspect@ab7818de7da2f20f
---
# Codex::inspect

Read-only check that returns ok or not-ok findings. In Hooks mode it reports one finding per event, checking that hooks.json has a matching command. In PullOnly mode it checks for the AGENTS.md block. It always checks that config.toml's mcp_servers.myrn command equals the binary path.

## Connections

- **Location:** `crates/myrn-adapters/src/codex.rs:276-332`
- **Signature:** `fn inspect(&self, ctx: &InstallCtx, mode: Mode) -> Vec<Finding>`
- **In:** [[file-crates-myrn-adapters-src-codex-rs]]
- **Calls:** [[adapter-rel]], [[adapter-read-json]], [[hooks-json-find]], [[codex-hooks-path]], [[codex-config-path]], [[codex-instructions-path]], [[codex-command-line]], [[pull-only-has-block]]
