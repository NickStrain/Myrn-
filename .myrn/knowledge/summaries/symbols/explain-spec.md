---
id: explain-spec
type: summary
status: unverified
title: explain::spec
source: myrn summarize
about: sym:crates/myrn-mcp/src/tools/explain.rs#tools::explain::spec
anchors:
- sym:crates/myrn-mcp/src/tools/explain.rs#tools::explain::spec@e538b4353e9ed435
map_version: '2'
---
# explain::spec

Builds the JSON tool definition for `explain`, with a description and an input schema that requires a `symbol` string.

## Behaviour

- no arguments -> JSON object with name "explain", a description, and inputSchema
- inputSchema marks "symbol" as required, typed as string
- accepted symbol forms per description: name, a::b or A.b qualified name, path, or path:line

## Terms

tool definition, input schema, tool metadata, MCP tool, json schema, tool registration

## Connections

- **Location:** `crates/myrn-mcp/src/tools/explain.rs:9-18`
- **Signature:** `pub fn spec() -> Value`
- **In:** [[file-crates-myrn-mcp-src-tools-explain-rs]]
- **Calls:** [[tools-schema]]
- **Called by:** [[tools-list]]
