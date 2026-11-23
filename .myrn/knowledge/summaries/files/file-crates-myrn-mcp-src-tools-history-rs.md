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

MCP history tool: lists indexed commits that touched a path, directory or symbol, newest first. Holds the tool descriptor (spec), argument parsing and the engine call (call), one-line-per-commit text formatting (render), and a days-since-epoch to YYYY-MM-DD converter (days_to_date).

## Connections

- **Location:** `crates/myrn-mcp/src/tools/history.rs`
- **In:** [[dir-crates-myrn-mcp-src-tools]]
- **Defines:** [[history-spec]], [[history-call]], [[history-render]], [[history-days-to-date]]
- **Imports:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-mcp-src-tools-mod-rs]]
- **Imported by:** [[file-crates-myrn-mcp-src-tools-mod-rs]]
