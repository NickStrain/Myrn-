---
id: file-crates-myrn-mcp-src-tools-impact-rs
type: summary
status: unverified
title: crates/myrn-mcp/src/tools/impact.rs
source: myrn summarize
about: file:crates/myrn-mcp/src/tools/impact.rs
anchors:
- file:crates/myrn-mcp/src/tools/impact.rs@9591d58a64c6f46f
---
# crates/myrn-mcp/src/tools/impact.rs

MCP tool handler for the `impact` tool: it declares the tool's JSON spec (name, description, and a required `symbol` string argument) and dispatches calls to the Engine's impact analysis, rendering up to 15 results.

## Connections

- **Location:** `crates/myrn-mcp/src/tools/impact.rs`
- **In:** [[dir-crates-myrn-mcp-src-tools]]
- **Defines:** [[impact-spec]], [[impact-call]]
- **Imports:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-mcp-src-tools-mod-rs]]
- **Imported by:** [[file-crates-myrn-mcp-src-tools-mod-rs]]
