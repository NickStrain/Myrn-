---
id: history-spec
type: summary
status: unverified
title: history::spec
source: myrn summarize
about: sym:crates/myrn-mcp/src/tools/history.rs#tools::history::spec
anchors:
- sym:crates/myrn-mcp/src/tools/history.rs#tools::history::spec@3d2f0fe4cfeee82d
map_version: '2'
---
# history::spec

Returns the JSON tool descriptor that advertises the history tool with a required path input and an optional integer limit.

## Behaviour

- takes no arguments and returns the same object on every call
- path is listed as required in inputSchema; limit's default of 15 appears only in its description text
- schema is built with the shared schema helper from the parent tools module

## Terms

tool schema, tool descriptor, input schema, MCP tool definition, tool listing, tool metadata

## Connections

- **Location:** `crates/myrn-mcp/src/tools/history.rs:9-18`
- **Signature:** `pub fn spec() -> Value`
- **In:** [[file-crates-myrn-mcp-src-tools-history-rs]]
- **Calls:** [[tools-schema]]
- **Called by:** [[tools-list]]
