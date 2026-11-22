---
id: dir-crates-myrn-mcp-tests
type: summary
status: unverified
title: crates/myrn-mcp/tests/
source: myrn summarize
about: module:crates/myrn-mcp/tests
anchors:
- module:crates/myrn-mcp/tests@01adff6f57d4f7ce
---
# crates/myrn-mcp/tests/

Integration test directory for the myrn-mcp server. protocol.rs exercises the JSON-RPC surface in-process: initialize handshake, notifications, tools/list, tools/call, and error codes, with its tools answering from an indexed git fixture repo. Helpers `engine` and `call` build that fixture and drive `handle_line`.

## Connections

- **In:** [[dir-crates-myrn-mcp]]
- **Files:** [[file-crates-myrn-mcp-tests-protocol-rs]]
