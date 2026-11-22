---
id: server-error
type: summary
status: unverified
title: server::error
source: myrn summarize
about: sym:crates/myrn-mcp/src/server.rs#server::error
anchors:
- sym:crates/myrn-mcp/src/server.rs#server::error@d9e3b4c27c18d6f9
---
# server::error

Builds a JSON-RPC 2.0 error response object with the given id, numeric code and message.

## Connections

- **Location:** `crates/myrn-mcp/src/server.rs:99-101`
- **Signature:** `fn error(id: Value, code: i64, message: &str) -> Value`
- **In:** [[file-crates-myrn-mcp-src-server-rs]]
- **Called by:** [[server-handle-line]], [[server-handle]]
