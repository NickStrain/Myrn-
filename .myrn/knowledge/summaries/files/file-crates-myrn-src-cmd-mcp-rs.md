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

Implements the `myrn mcp` CLI subcommand, which starts the MCP server over stdio. Its only item is `run`, which builds the engine from the shared `Ctx` and hands it to `myrn_mcp::serve`.

## Connections

- **Location:** `crates/myrn/src/cmd/mcp.rs`
- **In:** [[dir-crates-myrn-src-cmd]]
- **Defines:** [[mcp-run]]
- **Imports:** [[file-crates-myrn-src-cmd-mod-rs]]
- **Imported by:** [[file-crates-myrn-src-cmd-mod-rs]]
