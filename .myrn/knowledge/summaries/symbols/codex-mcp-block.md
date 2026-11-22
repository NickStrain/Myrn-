---
id: codex-mcp-block
type: summary
status: unverified
title: codex::mcp_block
source: myrn summarize
about: sym:crates/myrn-adapters/src/codex.rs#codex::mcp_block
anchors:
- sym:crates/myrn-adapters/src/codex.rs#codex::mcp_block@bade39464f9a5374
---
# codex::mcp_block

Renders the managed TOML block between the myrn start and end markers, with the binary path TOML-escaped as the command and args set to mcp.

## Connections

- **Location:** `crates/myrn-adapters/src/codex.rs:56-59`
- **Signature:** `fn mcp_block(binary: &Path) -> String`
- **In:** [[file-crates-myrn-adapters-src-codex-rs]]
- **Called by:** [[codex-write-mcp]]
