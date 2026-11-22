---
id: dir-crates-myrn-mcp-src-tools
type: summary
status: unverified
title: crates/myrn-mcp/src/tools/
source: myrn summarize
about: module:crates/myrn-mcp/src/tools
anchors:
- module:crates/myrn-mcp/src/tools@64d39a196620ff08
---
# crates/myrn-mcp/src/tools/

MCP tool handlers for the myrn-mcp server, one submodule per tool (context, explain, history, impact, recall, record). `mod.rs` declares the submodules and dispatches `list` and `call`; each module exposes `spec()` for its JSON schema and a handler that forwards arguments to the Engine and renders the result as text. `resolve.rs` is a doc-comment-only placeholder and is not exposed as a tool.

## Connections

- **In:** [[dir-crates-myrn-mcp-src]]
- **Files:** [[file-crates-myrn-mcp-src-tools-context-rs]], [[file-crates-myrn-mcp-src-tools-explain-rs]], [[file-crates-myrn-mcp-src-tools-history-rs]], [[file-crates-myrn-mcp-src-tools-impact-rs]], [[file-crates-myrn-mcp-src-tools-mod-rs]], [[file-crates-myrn-mcp-src-tools-recall-rs]], [[file-crates-myrn-mcp-src-tools-record-rs]], [[file-crates-myrn-mcp-src-tools-resolve-rs]]
