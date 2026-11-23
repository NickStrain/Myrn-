---
id: dir-crates-myrn-mcp
type: summary
status: unverified
title: crates/myrn-mcp/
source: myrn summarize
about: module:crates/myrn-mcp
anchors:
- module:crates/myrn-mcp@1a737bc7c402f557
---
# crates/myrn-mcp/

Rust MCP server crate that speaks JSON-RPC 2.0 over stdio, one message per line, with protocol on stdout and diagnostics on stderr. `src/lib.rs` re-exports `server::serve` as the entry point; `src/server.rs` runs the stdin loop, dispatches single and batch messages, and handles initialize, ping, tools/list and tools/call, while `src/tools/` holds one handler per tool and builds the advertised tool list, refreshes the index and marks cited node ids as used before routing each call. `tests/protocol.rs` runs integration tests against a temporary git repo with a small Rust file, covering the handshake, tool names, graph-backed tool answers, and error codes -32602, -32601 and -32700.

## Connections

- **In:** [[dir-crates]]
- **Directories:** [[dir-crates-myrn-mcp-src]], [[dir-crates-myrn-mcp-tests]]
