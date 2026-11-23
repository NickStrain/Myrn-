---
id: codex-mcp-block
type: summary
status: unverified
title: codex::mcp_block
source: myrn summarize
about: sym:crates/myrn-adapters/src/codex.rs#codex::mcp_block
anchors:
- sym:crates/myrn-adapters/src/codex.rs#codex::mcp_block@bade39464f9a5374
map_version: '2'
---
# codex::mcp_block

Renders the managed TOML block: start marker, [mcp_servers.myrn] table with command set to the binary path and args = ["mcp"], then end marker.

## Behaviour

- the binary path is TOML-escaped as a string
- the output ends with a newline after the end marker
- pure string building, no I/O

## Terms

mcp server table, toml snippet, managed config block

## Connections

- **Location:** `crates/myrn-adapters/src/codex.rs:56-59`
- **Signature:** `fn mcp_block(binary: &Path) -> String`
- **In:** [[file-crates-myrn-adapters-src-codex-rs]]
- **Called by:** [[codex-write-mcp]]
