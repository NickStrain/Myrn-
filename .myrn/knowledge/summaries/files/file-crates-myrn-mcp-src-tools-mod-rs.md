---
id: file-crates-myrn-mcp-src-tools-mod-rs
type: summary
status: unverified
title: crates/myrn-mcp/src/tools/mod.rs
source: myrn summarize
about: file:crates/myrn-mcp/src/tools/mod.rs
anchors:
- file:crates/myrn-mcp/src/tools/mod.rs@a8b7c95112a5beae
---
# crates/myrn-mcp/src/tools/mod.rs

Declares one submodule per MCP tool (context, explain, impact, history, recall, record, resolve) and dispatches tool calls to them. `resolve` is declared but is not included in `list` or `call`, so it is not exposed as a tool.

## Connections

- **Location:** `crates/myrn-mcp/src/tools/mod.rs`
- **In:** [[dir-crates-myrn-mcp-src-tools]]
- **Defines:** [[tools-list]], [[tools-call]], [[tools-str-arg]], [[tools-schema]]
- **Imports:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-mcp-src-tools-context-rs]], [[file-crates-myrn-mcp-src-tools-explain-rs]], [[file-crates-myrn-mcp-src-tools-history-rs]], [[file-crates-myrn-mcp-src-tools-impact-rs]], [[file-crates-myrn-mcp-src-tools-recall-rs]], [[file-crates-myrn-mcp-src-tools-record-rs]], [[file-crates-myrn-mcp-src-tools-resolve-rs]]
- **Imported by:** [[file-crates-myrn-mcp-src-lib-rs]], [[file-crates-myrn-mcp-src-server-rs]], [[file-crates-myrn-mcp-src-tools-context-rs]], [[file-crates-myrn-mcp-src-tools-explain-rs]], [[file-crates-myrn-mcp-src-tools-history-rs]], [[file-crates-myrn-mcp-src-tools-impact-rs]], [[file-crates-myrn-mcp-src-tools-recall-rs]], [[file-crates-myrn-mcp-src-tools-record-rs]]
