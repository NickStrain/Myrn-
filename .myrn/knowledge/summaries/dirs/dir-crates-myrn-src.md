---
id: dir-crates-myrn-src
type: summary
status: unverified
title: crates/myrn/src/
source: myrn summarize
about: module:crates/myrn/src
anchors:
- module:crates/myrn/src@f4fd1de925f31dd3
---
# crates/myrn/src/

Source root of the `myrn` CLI binary: main.rs defines the clap argument model (Cli, Command, DebugCommand), builds a cmd::Ctx from global flags, and dispatches each subcommand, including the `mcp` subcommand that runs the MCP server. Handler modules live in cmd/, where mod.rs declares the submodules and the shared Ctx (engine access plus emit mode), and each handler typically builds an engine from Ctx, calls one engine method, and prints text or JSON via ctx.emit; hook.rs is the never-fail entry point for agent hook scripts, while diff, forget, gc, publish, trace, and verify are declared but contain only doc comments.

## Connections

- **In:** [[dir-crates-myrn]]
- **Directories:** [[dir-crates-myrn-src-cmd]]
- **Files:** [[file-crates-myrn-src-main-rs]]
