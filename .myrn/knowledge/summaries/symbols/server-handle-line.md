---
id: server-handle-line
type: summary
status: unverified
title: server::handle_line
source: myrn summarize
about: sym:crates/myrn-mcp/src/server.rs#server::handle_line
anchors:
- sym:crates/myrn-mcp/src/server.rs#server::handle_line@a283426f6445cba5
map_version: '2'
---
# server::handle_line

Parses one input line as JSON. A top-level array is dispatched element by element, and any other value goes to handle.

## Behaviour

- invalid JSON -> error with id null, code -32700, message starting with parse error
- empty array, or every element yields no response -> None
- batch -> array of only the responses; notifications and non-object elements are dropped silently

## Terms

batch, JSON-RPC batch, parse error, request line, array of requests, batch response

## Connections

- **Location:** `crates/myrn-mcp/src/server.rs:36-50`
- **Signature:** `pub fn handle_line(engine: &mut Engine, line: &str) -> Option<Value>`
- **In:** [[file-crates-myrn-mcp-src-server-rs]]
- **Calls:** [[server-handle]], [[server-error]]
- **Called by:** [[server-serve]], [[call]]
- **Tests:** `handshake_list_and_call`
