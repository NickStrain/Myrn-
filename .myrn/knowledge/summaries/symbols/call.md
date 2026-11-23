---
id: call
type: summary
status: unverified
title: call
source: myrn summarize
about: sym:crates/myrn-mcp/tests/protocol.rs#call
anchors:
- sym:crates/myrn-mcp/tests/protocol.rs#call@8b4d7ab925c74e47
map_version: '2'
---
# call

Serialises a JSON-RPC message to a string, passes it to handle_line, and returns the parsed response, panicking if there is none.

## Behaviour

- json object -> handle_line response as serde_json Value
- notification with no id (handle_line returns None) -> panics with 'a response'
- a message is always sent as one line, since to_string produces compact JSON

## Terms

send request, json-rpc request, dispatch, test helper, invoke handler, send message

## Connections

- **Location:** `crates/myrn-mcp/tests/protocol.rs:29-31`
- **Signature:** `fn call(e: &mut Engine, msg: Value) -> Value`
- **In:** [[file-crates-myrn-mcp-tests-protocol-rs]]
- **Calls:** [[server-handle-line]]
- **Tests:** `handshake_list_and_call`
