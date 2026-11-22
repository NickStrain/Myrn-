---
id: server-handle
type: summary
status: unverified
title: server::handle
source: myrn summarize
about: sym:crates/myrn-mcp/src/server.rs#server::handle
anchors:
- sym:crates/myrn-mcp/src/server.rs#server::handle@8af4d0f49aa47f82
---
# server::handle

Routes a single message by method and returns None for notifications (messages without an id); initialize echoes the client's protocolVersion if it is supported and otherwise falls back to 2025-06-18, tool execution failures come back as successful results with isError true, an unknown tool name yields error -32602, and an unknown method yields error -32601.

## Connections

- **Location:** `crates/myrn-mcp/src/server.rs:52-97`
- **Signature:** `fn handle(engine: &mut Engine, msg: &Value) -> Option<Value>`
- **In:** [[file-crates-myrn-mcp-src-server-rs]]
- **Calls:** [[server-error]], [[tools-list]], [[tools-call]]
- **Called by:** [[server-handle-line]]
