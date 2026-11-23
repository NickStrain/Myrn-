---
id: context-spec
type: summary
status: unverified
title: context::spec
source: myrn summarize
about: sym:crates/myrn-mcp/src/tools/context.rs#tools::context::spec
anchors:
- sym:crates/myrn-mcp/src/tools/context.rs#tools::context::spec@87c6af4f8a26a40d
map_version: '2'
---
# context::spec

Returns the JSON tool definition for `context`, with its description and an input schema that requires `task` (string) and accepts an optional integer `budget`.

## Behaviour

- no arguments -> the same fixed JSON every call; nothing is read from input
- schema lists only `task` as required; `budget` is optional and documented as default 1500
- the default 1500 appears only in the description text; this function does not enforce it

## Terms

tool definition, tool schema, MCP tool listing, input schema, tool description, task parameter, token budget parameter

## Connections

- **Location:** `crates/myrn-mcp/src/tools/context.rs:9-19`
- **Signature:** `pub fn spec() -> Value`
- **In:** [[file-crates-myrn-mcp-src-tools-context-rs]]
- **Calls:** [[tools-schema]]
- **Called by:** [[tools-list]]
