---
id: file-crates-myrn-mcp-src-tools-history-rs
type: summary
status: unverified
title: crates/myrn-mcp/src/tools/history.rs
source: myrn summarize
about: file:crates/myrn-mcp/src/tools/history.rs
anchors:
- file:crates/myrn-mcp/src/tools/history.rs@c38e6c40a2f272f8
---
# crates/myrn-mcp/src/tools/history.rs

MCP tool module for the `history` tool. It defines the tool's JSON spec, parses the `path` and `limit` arguments, calls `Engine::history`, and renders the returned entries as plain text lines, with a helper that converts epoch days to a calendar date.

## Connections

- **Location:** `crates/myrn-mcp/src/tools/history.rs`
- **In:** [[dir-crates-myrn-mcp-src-tools]]
- **Defines:** [[history-spec]], [[history-call]], [[history-render]], [[history-days-to-date]]
- **Imports:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-mcp-src-tools-mod-rs]]
- **Imported by:** [[file-crates-myrn-mcp-src-tools-mod-rs]]
