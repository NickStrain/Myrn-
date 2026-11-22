---
id: file-crates-myrn-mcp-src-tools-explain-rs
type: summary
status: unverified
title: crates/myrn-mcp/src/tools/explain.rs
source: myrn summarize
about: file:crates/myrn-mcp/src/tools/explain.rs
anchors:
- file:crates/myrn-mcp/src/tools/explain.rs@8b22d492c54348ee
---
# crates/myrn-mcp/src/tools/explain.rs

MCP tool definition for the `explain` tool, which returns a symbol or file card from the engine. It exposes `spec()` for the tool's JSON schema and `call()` as the handler that runs the lookup and renders the result.

## Connections

- **Location:** `crates/myrn-mcp/src/tools/explain.rs`
- **In:** [[dir-crates-myrn-mcp-src-tools]]
- **Defines:** [[explain-spec]], [[explain-call]]
- **Imports:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-mcp-src-tools-mod-rs]]
- **Imported by:** [[file-crates-myrn-mcp-src-tools-mod-rs]]
