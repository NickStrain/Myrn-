---
id: file-crates-myrn-src-cmd-mcp-rs
type: summary
status: unverified
title: crates/myrn/src/cmd/mcp.rs
source: myrn summarize
about: file:crates/myrn/src/cmd/mcp.rs
anchors:
- file:crates/myrn/src/cmd/mcp.rs@059f4d65f70faaa7
---
# crates/myrn/src/cmd/mcp.rs

Defines the `myrn mcp` subcommand, which starts the MCP (Model Context Protocol) server over stdio. It builds the engine from the shared command context and hands it to the `myrn_mcp` crate's serve function.

## Connections

- **Location:** `crates/myrn/src/cmd/mcp.rs`
- **In:** [[dir-crates-myrn-src-cmd]]
- **Defines:** [[mcp-run]]
- **Imports:** [[file-crates-myrn-src-cmd-mod-rs]]
- **Imported by:** [[file-crates-myrn-src-cmd-mod-rs]]
