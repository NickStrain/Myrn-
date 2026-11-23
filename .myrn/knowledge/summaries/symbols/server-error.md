---
id: server-error
type: summary
status: unverified
title: server::error
source: myrn summarize
about: sym:crates/myrn-mcp/src/server.rs#server::error
anchors:
- sym:crates/myrn-mcp/src/server.rs#server::error@d9e3b4c27c18d6f9
map_version: '2'
---
# server::error

Builds a JSON-RPC 2.0 error response object from the given id, numeric code and message.

## Behaviour

- error(Value::Null, -32700, msg) -> id null, used for parse errors
- id is copied as given with no validation
- code is passed through as i64 with no range check

## Terms

error envelope, error response, JSON-RPC error, error object, error code, error builder

## Connections

- **Location:** `crates/myrn-mcp/src/server.rs:99-101`
- **Signature:** `fn error(id: Value, code: i64, message: &str) -> Value`
- **In:** [[file-crates-myrn-mcp-src-server-rs]]
- **Called by:** [[server-handle-line]], [[server-handle]]
