---
id: file-crates-myrn-mcp-src-tools-supersede-rs
type: summary
status: unverified
title: crates/myrn-mcp/src/tools/supersede.rs
source: myrn summarize
about: file:crates/myrn-mcp/src/tools/supersede.rs
anchors:
- file:crates/myrn-mcp/src/tools/supersede.rs@1cd18c87369899b7
---
# crates/myrn-mcp/src/tools/supersede.rs

MCP tool handler that exposes the `supersede` tool: it defines the tool's JSON schema (`spec`) and the call handler (`call`) that validates `id` and `item` and delegates to `Engine::supersede`, rendering the result as a string. Replaced records stay in history but are hidden from normal results.

## Connections

- **Location:** `crates/myrn-mcp/src/tools/supersede.rs`
- **In:** [[dir-crates-myrn-mcp-src-tools]]
- **Defines:** [[supersede-spec]], [[supersede-call]]
- **Imports:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-mcp-src-tools-mod-rs]]
- **Imported by:** [[file-crates-myrn-mcp-src-tools-mod-rs]]
