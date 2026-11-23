---
id: file-crates-myrn-mcp-src-server-rs
type: summary
status: unverified
title: crates/myrn-mcp/src/server.rs
source: myrn summarize
about: file:crates/myrn-mcp/src/server.rs
anchors:
- file:crates/myrn-mcp/src/server.rs@86aa0fd7e7742b8e
---
# crates/myrn-mcp/src/server.rs

JSON-RPC 2.0 server over stdio, one message per line, with protocol output on stdout and diagnostics on stderr. Contains the stdin loop, batch and single-message dispatch, the initialize, ping, tools/list and tools/call handlers, and the error envelope builder.

## Connections

- **Location:** `crates/myrn-mcp/src/server.rs`
- **In:** [[dir-crates-myrn-mcp-src]]
- **Defines:** [[server-serve]], [[server-handle-line]], [[server-handle]], [[server-error]]
- **Imports:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-mcp-src-tools-mod-rs]]
- **Imported by:** [[file-crates-myrn-mcp-src-lib-rs]], [[file-crates-myrn-mcp-tests-protocol-rs]]
