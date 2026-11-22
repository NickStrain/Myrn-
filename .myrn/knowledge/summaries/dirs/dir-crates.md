---
id: dir-crates
type: summary
status: unverified
title: crates/
source: myrn summarize
about: module:crates
anchors:
- module:crates@96456c87a3220d92
---
# crates/

Rust workspace for the myrn code-map tool: myrn is the CLI binary, myrn-mcp is the stdio JSON-RPC MCP server agents spawn, and myrn-engine is the core library that both call through its Engine API in api.rs. Shared domain types live in myrn-model (pure, no I/O), persistence in myrn-store (SQLite plus Markdown notes and JSONL episode logs), per-agent hook and config integration in myrn-adapters, and the optional long-lived embedding helper in myrn-embed.

## Connections

- **In:** [[project]]
- **Directories:** [[dir-crates-myrn]], [[dir-crates-myrn-adapters]], [[dir-crates-myrn-embed]], [[dir-crates-myrn-engine]], [[dir-crates-myrn-mcp]], [[dir-crates-myrn-model]], [[dir-crates-myrn-store]]
