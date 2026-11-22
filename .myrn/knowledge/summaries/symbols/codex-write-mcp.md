---
id: codex-write-mcp
type: summary
status: unverified
title: codex::write_mcp
source: myrn summarize
about: sym:crates/myrn-adapters/src/codex.rs#codex::write_mcp
anchors:
- sym:crates/myrn-adapters/src/codex.rs#codex::write_mcp@4886575ffe51f869
---
# codex::write_mcp

Replaces or appends the managed block in config.toml and creates .codex/ if needed. It fails if a [mcp_servers.myrn] table exists outside the managed markers, and the user must remove that table manually.

## Connections

- **Location:** `crates/myrn-adapters/src/codex.rs:82-105`
- **Signature:** `fn write_mcp(root: &Path, binary: &Path) -> Result<()>`
- **In:** [[file-crates-myrn-adapters-src-codex-rs]]
- **Calls:** [[codex-codex-dir]], [[codex-config-path]], [[codex-mcp-block]], [[codex-strip-mcp]]
- **Called by:** [[codex-install]]
