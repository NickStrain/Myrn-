---
id: server-handle-line
type: summary
status: unverified
title: server::handle_line
source: myrn summarize
about: sym:crates/myrn-mcp/src/server.rs#server::handle_line
anchors:
- sym:crates/myrn-mcp/src/server.rs#server::handle_line@a283426f6445cba5
---
# server::handle_line

Parses one input line as JSON and returns the response Value, or None when nothing should be sent: a parse failure yields a -32700 error with a null id, and a top-level array is handled as a batch where notifications are dropped and None is returned if no responses remain.

## Connections

- **Location:** `crates/myrn-mcp/src/server.rs:36-50`
- **Signature:** `pub fn handle_line(engine: &mut Engine, line: &str) -> Option<Value>`
- **In:** [[file-crates-myrn-mcp-src-server-rs]]
- **Calls:** [[server-handle]], [[server-error]]
- **Called by:** [[server-serve]], [[call]]
- **Tests:** `handshake_list_and_call`
