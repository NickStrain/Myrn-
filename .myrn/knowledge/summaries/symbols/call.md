---
id: call
type: summary
status: unverified
title: call
source: myrn summarize
about: sym:crates/myrn-mcp/tests/protocol.rs#call
anchors:
- sym:crates/myrn-mcp/tests/protocol.rs#call@8b4d7ab925c74e47
---
# call

Serializes a JSON-RPC message and passes it to `handle_line`, returning the parsed response Value; panics if the server returns no response (i.e. for notifications).

## Connections

- **Location:** `crates/myrn-mcp/tests/protocol.rs:29-31`
- **Signature:** `fn call(e: &mut Engine, msg: Value) -> Value`
- **In:** [[file-crates-myrn-mcp-tests-protocol-rs]]
- **Calls:** [[server-handle-line]]
- **Tests:** `handshake_list_and_call`
