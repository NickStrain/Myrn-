---
id: file-crates-myrn-adapters-src-codex-rs
type: summary
status: unverified
title: crates/myrn-adapters/src/codex.rs
source: myrn summarize
about: file:crates/myrn-adapters/src/codex.rs
anchors:
- file:crates/myrn-adapters/src/codex.rs@6c2c5b786d140f1e
---
# crates/myrn-adapters/src/codex.rs

Codex adapter for myrn. It writes and removes the myrn hooks in .codex/hooks.json, the managed [mcp_servers.myrn] block in .codex/config.toml, and the AGENTS.md instruction block. It also converts Codex hook payloads into myrn events and formats hook output for Codex.

## Connections

- **Location:** `crates/myrn-adapters/src/codex.rs`
- **In:** [[dir-crates-myrn-adapters-src]]
- **Defines:** [[codex-codex]], [[codex-codex-dir]], [[codex-hooks-path]], [[codex-config-path]], [[codex-instructions-path]], [[codex-command-line]], [[codex-is-ours]], [[codex-mcp-block]], [[codex-strip-mcp]], [[codex-write-mcp]], [[codex-patch-files]], [[codex-failure-text]], [[codex-name]], [[codex-display-name]], [[codex-detect]], [[codex-install]], [[codex-uninstall]], [[codex-inspect]], [[codex-translate]], [[codex-format]], [[tests-ctx-codex]], [[tests-payload-codex]]
- **Imports:** [[file-crates-myrn-adapters-src-adapter-rs]], [[file-crates-myrn-adapters-src-detect-rs]], [[file-crates-myrn-adapters-src-lib-rs]], [[file-crates-myrn-adapters-src-pull-only-rs]], [[file-crates-myrn-model-src-event-rs]]
- **Imported by:** [[file-crates-myrn-adapters-src-lib-rs]]
