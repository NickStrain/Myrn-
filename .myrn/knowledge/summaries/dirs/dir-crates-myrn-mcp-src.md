---
id: dir-crates-myrn-mcp-src
type: summary
status: unverified
title: crates/myrn-mcp/src/
source: myrn summarize
about: module:crates/myrn-mcp/src
anchors:
- module:crates/myrn-mcp/src@54a7b70823fb0835
---
# crates/myrn-mcp/src/

Source for the myrn-mcp MCP server, which speaks JSON-RPC 2.0 over stdio, one message per line, with protocol on stdout and diagnostics on stderr; the agent spawns and owns the process. lib.rs declares the `server` and `tools` modules and re-exports `server::serve` as the entry point. server.rs holds the stdin loop, batch/single dispatch, and the initialize, ping, tools/list and tools/call handlers; tools/ holds one handler module per tool, and its mod.rs builds the advertised tool list, refreshes the index and marks cited node ids as used before routing each call.

## Connections

- **In:** [[dir-crates-myrn-mcp]]
- **Directories:** [[dir-crates-myrn-mcp-src-tools]]
- **Files:** [[file-crates-myrn-mcp-src-lib-rs]], [[file-crates-myrn-mcp-src-server-rs]]
