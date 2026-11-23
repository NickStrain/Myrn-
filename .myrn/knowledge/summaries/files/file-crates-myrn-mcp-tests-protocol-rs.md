---
id: file-crates-myrn-mcp-tests-protocol-rs
type: summary
status: unverified
title: crates/myrn-mcp/tests/protocol.rs
source: myrn summarize
about: file:crates/myrn-mcp/tests/protocol.rs
anchors:
- file:crates/myrn-mcp/tests/protocol.rs@eddea179d6222c06
---
# crates/myrn-mcp/tests/protocol.rs

Integration test for the MCP server's JSON-RPC layer. It builds a temporary git repo with a small Rust source file, then checks the initialize handshake, the tools/list names, tool calls answered from the graph, and the error codes -32602, -32601 and -32700. The helpers engine and call are at the top.

## Connections

- **Location:** `crates/myrn-mcp/tests/protocol.rs`
- **In:** [[dir-crates-myrn-mcp-tests]]
- **Defines:** [[engine]], [[call]]
- **Imports:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-mcp-src-server-rs]]
