---
id: codex-write-mcp
type: summary
status: unverified
title: codex::write_mcp
source: myrn summarize
about: sym:crates/myrn-adapters/src/codex.rs#codex::write_mcp
anchors:
- sym:crates/myrn-adapters/src/codex.rs#codex::write_mcp@4886575ffe51f869
map_version: '2'
---
# codex::write_mcp

Writes or replaces the managed MCP block in config.toml, creating .codex/ if needed. It refuses to write if [mcp_servers.myrn] already exists outside the markers.

## Behaviour

- existing config with other tables -> kept, with the block appended after a blank line
- [mcp_servers.myrn] defined outside the markers -> error and the file is left unchanged
- no existing file -> created containing only the block

## Terms

write config, install mcp, toml write, conflict check

## Connections

- **Location:** `crates/myrn-adapters/src/codex.rs:82-105`
- **Signature:** `fn write_mcp(root: &Path, binary: &Path) -> Result<()>`
- **In:** [[file-crates-myrn-adapters-src-codex-rs]]
- **Calls:** [[codex-codex-dir]], [[codex-config-path]], [[codex-mcp-block]], [[codex-strip-mcp]]
- **Called by:** [[codex-install]]
