---
id: file-crates-myrn-mcp-src-tools-recall-rs
type: summary
status: unverified
title: crates/myrn-mcp/src/tools/recall.rs
source: myrn summarize
about: file:crates/myrn-mcp/src/tools/recall.rs
anchors:
- file:crates/myrn-mcp/src/tools/recall.rs@f764abc6b2c4755c
---
# crates/myrn-mcp/src/tools/recall.rs

Defines the `recall` MCP tool, which takes an error message or failing test output and returns matching past bug records with root cause, responsible code, and fix. It exposes the tool's JSON spec (name, description, input schema with required `error` and optional `budget`) and the handler that runs the lookup and logs the injection.

## Connections

- **Location:** `crates/myrn-mcp/src/tools/recall.rs`
- **In:** [[dir-crates-myrn-mcp-src-tools]]
- **Defines:** [[recall-spec]], [[recall-call]]
- **Imports:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-mcp-src-tools-mod-rs]]
- **Imported by:** [[file-crates-myrn-mcp-src-tools-mod-rs]]
