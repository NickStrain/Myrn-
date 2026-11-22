---
id: context-spec
type: summary
status: unverified
title: context::spec
source: myrn summarize
about: sym:crates/myrn-mcp/src/tools/context.rs#tools::context::spec
anchors:
- sym:crates/myrn-mcp/src/tools/context.rs#tools::context::spec@87c6af4f8a26a40d
---
# context::spec

Returns the MCP tool descriptor JSON with name `context`, a description telling agents to call it before searching, and an input schema with required string `task` and optional integer `budget`. Takes no arguments and has no side effects. The schema text says the default budget is 1500, but this function does not set it; the default is whatever the engine applies when `budget` is absent.

## Connections

- **Location:** `crates/myrn-mcp/src/tools/context.rs:9-19`
- **Signature:** `pub fn spec() -> Value`
- **In:** [[file-crates-myrn-mcp-src-tools-context-rs]]
- **Calls:** [[tools-schema]]
- **Called by:** [[tools-list]]
