---
id: file-crates-myrn-mcp-src-tools-mod-rs
type: summary
status: unverified
title: crates/myrn-mcp/src/tools/mod.rs
source: myrn summarize
about: file:crates/myrn-mcp/src/tools/mod.rs
anchors:
- file:crates/myrn-mcp/src/tools/mod.rs@860647202c1df62c
---
# crates/myrn-mcp/src/tools/mod.rs

Dispatch layer for the MCP tools. It declares one submodule per tool, builds the advertised spec list, and routes a named call to its handler after refreshing the index and marking any node ids the call cites as used. It also holds the argument helpers `str_arg` and `schema`.

## Connections

- **Location:** `crates/myrn-mcp/src/tools/mod.rs`
- **In:** [[dir-crates-myrn-mcp-src-tools]]
- **Defines:** [[tools-list]], [[tools-call]], [[tools-cited]], [[tools-str-arg]], [[tools-schema]]
- **Imports:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-mcp-src-tools-confirm-rs]], [[file-crates-myrn-mcp-src-tools-context-rs]], [[file-crates-myrn-mcp-src-tools-explain-rs]], [[file-crates-myrn-mcp-src-tools-history-rs]], [[file-crates-myrn-mcp-src-tools-impact-rs]], [[file-crates-myrn-mcp-src-tools-recall-rs]], [[file-crates-myrn-mcp-src-tools-record-rs]], [[file-crates-myrn-mcp-src-tools-resolve-rs]], [[file-crates-myrn-mcp-src-tools-supersede-rs]]
- **Imported by:** [[file-crates-myrn-mcp-src-lib-rs]], [[file-crates-myrn-mcp-src-server-rs]], [[file-crates-myrn-mcp-src-tools-confirm-rs]], [[file-crates-myrn-mcp-src-tools-context-rs]], [[file-crates-myrn-mcp-src-tools-explain-rs]], [[file-crates-myrn-mcp-src-tools-history-rs]], [[file-crates-myrn-mcp-src-tools-impact-rs]], [[file-crates-myrn-mcp-src-tools-recall-rs]], [[file-crates-myrn-mcp-src-tools-record-rs]], [[file-crates-myrn-mcp-src-tools-supersede-rs]]
