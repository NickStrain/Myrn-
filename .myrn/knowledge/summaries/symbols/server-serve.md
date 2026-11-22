---
id: server-serve
type: summary
status: unverified
title: server::serve
source: myrn summarize
about: sym:crates/myrn-mcp/src/server.rs#server::serve
anchors:
- sym:crates/myrn-mcp/src/server.rs#server::serve@385d91a4e6762fd5
---
# server::serve

Takes ownership of the Engine and loops over stdin lines until EOF, skipping blank lines and writing each non-empty response as one line to stdout followed by a flush; read or write failures propagate as errors.

## Connections

- **Location:** `crates/myrn-mcp/src/server.rs:19-33`
- **Signature:** `pub fn serve(mut engine: Engine) -> Result<()>`
- **In:** [[file-crates-myrn-mcp-src-server-rs]]
- **Calls:** [[server-handle-line]]
- **Called by:** [[mcp-run]]
