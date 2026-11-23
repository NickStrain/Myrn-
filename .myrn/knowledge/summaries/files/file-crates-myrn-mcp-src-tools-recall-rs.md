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

MCP tool handler for 'recall', which looks up known past bugs matching an error message or failing test output. It holds the tool's JSON spec (spec) and its call handler (call), which delegates to Engine::recall_text and logs matched ids.

## Connections

- **Location:** `crates/myrn-mcp/src/tools/recall.rs`
- **In:** [[dir-crates-myrn-mcp-src-tools]]
- **Defines:** [[recall-spec]], [[recall-call]]
- **Imports:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-mcp-src-tools-mod-rs]]
- **Imported by:** [[file-crates-myrn-mcp-src-tools-mod-rs]]
