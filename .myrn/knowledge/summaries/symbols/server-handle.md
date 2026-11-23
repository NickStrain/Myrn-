---
id: server-handle
type: summary
status: unverified
title: server::handle
source: myrn summarize
about: sym:crates/myrn-mcp/src/server.rs#server::handle
anchors:
- sym:crates/myrn-mcp/src/server.rs#server::handle@8af4d0f49aa47f82
map_version: '2'
---
# server::handle

Routes one JSON-RPC message by method name (initialize, ping, tools/list, tools/call) and returns None when the message has no id.

## Behaviour

- no id field -> None, no response; id null is still a request and gets a response
- initialize with a missing or unsupported protocolVersion -> answers 2025-06-18; a supported version is echoed back
- tools/call tool error -> success result with isError true; unknown tool -> -32602; unknown method -> -32601

## Terms

dispatch, method router, notification, protocol version negotiation, handshake, tool call, tools list, unknown method

## Connections

- **Location:** `crates/myrn-mcp/src/server.rs:52-97`
- **Signature:** `fn handle(engine: &mut Engine, msg: &Value) -> Option<Value>`
- **In:** [[file-crates-myrn-mcp-src-server-rs]]
- **Calls:** [[server-error]], [[tools-list]], [[tools-call]]
- **Called by:** [[server-handle-line]]
