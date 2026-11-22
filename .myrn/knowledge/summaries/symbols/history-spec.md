---
id: history-spec
type: summary
status: unverified
title: history::spec
source: myrn summarize
about: sym:crates/myrn-mcp/src/tools/history.rs#tools::history::spec
anchors:
- sym:crates/myrn-mcp/src/tools/history.rs#tools::history::spec@3d2f0fe4cfeee82d
---
# history::spec

Returns the MCP tool definition JSON with name `history`, a description, and an input schema where `path` is required (string) and `limit` is an optional integer described as defaulting to 15.

## Connections

- **Location:** `crates/myrn-mcp/src/tools/history.rs:9-18`
- **Signature:** `pub fn spec() -> Value`
- **In:** [[file-crates-myrn-mcp-src-tools-history-rs]]
- **Calls:** [[tools-schema]]
- **Called by:** [[tools-list]]
