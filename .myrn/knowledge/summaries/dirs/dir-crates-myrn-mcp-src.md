---
id: dir-crates-myrn-mcp-src
type: summary
status: unverified
title: crates/myrn-mcp/src/
source: myrn summarize
about: module:crates/myrn-mcp/src
anchors:
- module:crates/myrn-mcp/src@4c259e2ba1525095
---
# crates/myrn-mcp/src/

Source for the myrn-mcp MCP server, a stdio JSON-RPC 2.0 server that the agent spawns and owns. lib.rs declares the `server` and `tools` modules and re-exports `server::serve` as the entry point; server.rs reads one message per line, dispatches initialize, ping, tools/list and tools/call, and writes protocol output only to stdout. tools/mod.rs routes list and call to one submodule per tool (context, explain, history, impact, recall, record), each exposing spec() for its JSON schema and a handler that forwards arguments to the Engine and returns text; resolve.rs is a doc-comment-only placeholder not exposed as a tool.

## Connections

- **In:** [[dir-crates-myrn-mcp]]
- **Directories:** [[dir-crates-myrn-mcp-src-tools]]
- **Files:** [[file-crates-myrn-mcp-src-lib-rs]], [[file-crates-myrn-mcp-src-server-rs]]
