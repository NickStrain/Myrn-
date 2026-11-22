---
id: dir-crates-myrn-mcp
type: summary
status: unverified
title: crates/myrn-mcp/
source: myrn summarize
about: module:crates/myrn-mcp
anchors:
- module:crates/myrn-mcp@c94a2d71bbf184b2
---
# crates/myrn-mcp/

Stdio JSON-RPC 2.0 MCP server (myrn-mcp) that an agent spawns and owns; src/server.rs reads one message per line and handles initialize, ping, tools/list and tools/call, writing protocol output only to stdout, while src/tools/ routes each tool (context, explain, history, impact, recall, record) to its own submodule that forwards arguments to the Engine. tests/protocol.rs drives the server in-process via handle_line against an indexed git fixture repo built by the engine and call helpers.

## Connections

- **In:** [[dir-crates]]
- **Directories:** [[dir-crates-myrn-mcp-src]], [[dir-crates-myrn-mcp-tests]]
