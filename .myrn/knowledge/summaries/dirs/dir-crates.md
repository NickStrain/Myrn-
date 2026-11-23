---
id: dir-crates
type: summary
status: unverified
title: crates/
source: myrn summarize
about: module:crates
anchors:
- module:crates@9456683f1c77de73
---
# crates/

crates/ is the Rust workspace for Myrn, a code-context tool, split into a pure domain model (myrn-model), storage (myrn-store), the core engine (myrn-engine), agent adapters (myrn-adapters), optional embeddings (myrn-embed), an MCP JSON-RPC server (myrn-mcp), and the myrn CLI binary (myrn), which dispatches thin subcommand handlers to the engine. Adapters must reach storage only through the engine's api.rs, and the MCP server and CLI both sit on top of the engine rather than touching stores directly.

## Connections

- **In:** [[project]]
- **Directories:** [[dir-crates-myrn]], [[dir-crates-myrn-adapters]], [[dir-crates-myrn-embed]], [[dir-crates-myrn-engine]], [[dir-crates-myrn-mcp]], [[dir-crates-myrn-model]], [[dir-crates-myrn-store]]
