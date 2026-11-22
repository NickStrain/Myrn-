---
id: file-crates-myrn-mcp-tests-protocol-rs
type: summary
status: unverified
title: crates/myrn-mcp/tests/protocol.rs
source: myrn summarize
about: file:crates/myrn-mcp/tests/protocol.rs
anchors:
- file:crates/myrn-mcp/tests/protocol.rs@1512ba7760e0bbf1
---
# crates/myrn-mcp/tests/protocol.rs

Integration test for the myrn-mcp server, checking that it speaks JSON-RPC correctly (initialize handshake, notifications, tools/list, tools/call, and error codes) and that its tools answer from an indexed git repo. Helpers `engine` and `call` build a fixture repo and drive `handle_line` in-process.

## Connections

- **Location:** `crates/myrn-mcp/tests/protocol.rs`
- **In:** [[dir-crates-myrn-mcp-tests]]
- **Defines:** [[engine]], [[call]]
- **Imports:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-mcp-src-server-rs]]
