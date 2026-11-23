---
id: dir-crates-myrn-mcp-src-tools
type: summary
status: unverified
title: crates/myrn-mcp/src/tools/
source: myrn summarize
about: module:crates/myrn-mcp/src/tools
anchors:
- module:crates/myrn-mcp/src/tools@6256acaf3327d158
---
# crates/myrn-mcp/src/tools/

Holds the MCP tool handlers for the myrn-mcp server, one module per tool (context, explain, impact, history, recall, record, confirm, supersede; resolve is doc-only). mod.rs is the dispatcher: it builds the advertised spec list, refreshes the index and marks cited node ids as used before routing a call to the matching handler, and provides the str_arg and schema helpers each tool uses to parse arguments and define its input schema.

## Connections

- **In:** [[dir-crates-myrn-mcp-src]]
- **Files:** [[file-crates-myrn-mcp-src-tools-confirm-rs]], [[file-crates-myrn-mcp-src-tools-context-rs]], [[file-crates-myrn-mcp-src-tools-explain-rs]], [[file-crates-myrn-mcp-src-tools-history-rs]], [[file-crates-myrn-mcp-src-tools-impact-rs]], [[file-crates-myrn-mcp-src-tools-mod-rs]], [[file-crates-myrn-mcp-src-tools-recall-rs]], [[file-crates-myrn-mcp-src-tools-record-rs]], [[file-crates-myrn-mcp-src-tools-resolve-rs]], [[file-crates-myrn-mcp-src-tools-supersede-rs]]
