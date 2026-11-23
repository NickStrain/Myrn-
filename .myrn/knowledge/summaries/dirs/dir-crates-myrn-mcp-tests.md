---
id: dir-crates-myrn-mcp-tests
type: summary
status: unverified
title: crates/myrn-mcp/tests/
source: myrn summarize
about: module:crates/myrn-mcp/tests
anchors:
- module:crates/myrn-mcp/tests@c59607e530603d81
---
# crates/myrn-mcp/tests/

Integration tests for the myrn-mcp server's JSON-RPC layer, run against a temporary git repo containing a small Rust source file. protocol.rs covers the initialize handshake, the tools/list names, tool calls answered from the graph, and the JSON-RPC error codes -32602, -32601 and -32700; the engine and call helpers at the top of the file set up the server and send requests.

## Connections

- **In:** [[dir-crates-myrn-mcp]]
- **Files:** [[file-crates-myrn-mcp-tests-protocol-rs]]
