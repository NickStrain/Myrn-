---
id: file-crates-myrn-src-main-rs
type: summary
status: unverified
title: crates/myrn/src/main.rs
source: myrn summarize
about: file:crates/myrn/src/main.rs
anchors:
- file:crates/myrn/src/main.rs@43f598045488335c
---
# crates/myrn/src/main.rs

Binary entry point for the myrn CLI, which also runs the MCP server via the mcp subcommand. It declares the clap argument model (Cli, Command, DebugCommand) and main, which builds a cmd::Ctx from the global flags and dispatches each subcommand to its handler in the cmd module.

## Connections

- **Location:** `crates/myrn/src/main.rs`
- **In:** [[dir-crates-myrn-src]]
- **Defines:** [[cli-main]], [[command]], [[debugcommand]], [[main-main]]
- **Imports:** [[file-crates-myrn-src-cmd-mod-rs]]
