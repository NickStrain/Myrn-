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

JSON-RPC 2.0 server over stdio that reads one message per line, dispatches initialize, ping, tools/list and tools/call to the tools module, and writes protocol responses only to stdout.

## Connections

- **Location:** `crates/myrn-mcp/src/server.rs`
- **In:** [[dir-crates-myrn-mcp-src]]
- **Defines:** [[server-serve]], [[server-handle-line]], [[server-handle]], [[server-error]]
- **Imports:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-mcp-src-tools-mod-rs]]
- **Imported by:** [[file-crates-myrn-mcp-src-lib-rs]], [[file-crates-myrn-mcp-tests-protocol-rs]]
