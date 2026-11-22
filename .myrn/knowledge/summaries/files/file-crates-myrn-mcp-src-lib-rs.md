---
id: file-crates-myrn-mcp-src-lib-rs
type: summary
status: unverified
title: crates/myrn-mcp/src/lib.rs
source: myrn summarize
about: file:crates/myrn-mcp/src/lib.rs
anchors:
- file:crates/myrn-mcp/src/lib.rs@e19aaec80aa575ad
---
# crates/myrn-mcp/src/lib.rs

Crate root for the myrn-mcp MCP server, which runs over stdio and is spawned and owned by the agent. It declares the `server` and `tools` modules and re-exports `server::serve` as the crate's entry point.

## Connections

- **Location:** `crates/myrn-mcp/src/lib.rs`
- **In:** [[dir-crates-myrn-mcp-src]]
- **Imports:** [[file-crates-myrn-mcp-src-server-rs]], [[file-crates-myrn-mcp-src-tools-mod-rs]]
