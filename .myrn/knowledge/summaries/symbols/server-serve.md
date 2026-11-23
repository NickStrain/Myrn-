---
id: server-serve
type: summary
status: unverified
title: server::serve
source: myrn summarize
about: sym:crates/myrn-mcp/src/server.rs#server::serve
anchors:
- sym:crates/myrn-mcp/src/server.rs#server::serve@385d91a4e6762fd5
map_version: '2'
---
# server::serve

Reads lines from stdin until EOF, skips blank lines, and writes each non-empty response to stdout as one JSON line, then flushes.

## Behaviour

- blank or whitespace-only line -> skipped, nothing written
- handle_line returns None (notification or empty batch) -> nothing written
- stdin closes -> Ok(()); a read or write io error -> returned as Err

## Terms

stdio loop, stdin reader, main loop, stdout writer, line reader, server loop

## Connections

- **Location:** `crates/myrn-mcp/src/server.rs:19-33`
- **Signature:** `pub fn serve(mut engine: Engine) -> Result<()>`
- **In:** [[file-crates-myrn-mcp-src-server-rs]]
- **Calls:** [[server-handle-line]]
- **Called by:** [[mcp-run]]
