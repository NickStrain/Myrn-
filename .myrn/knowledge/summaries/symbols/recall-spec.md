---
id: recall-spec
type: summary
status: unverified
title: recall::spec
source: myrn summarize
about: sym:crates/myrn-mcp/src/tools/recall.rs#tools::recall::spec
anchors:
- sym:crates/myrn-mcp/src/tools/recall.rs#tools::recall::spec@19f242e759a9802b
map_version: '2'
---
# recall::spec

Builds the JSON tool definition for 'recall', with a required 'error' string and an optional 'budget' integer.

## Behaviour

- no args -> JSON object with name 'recall', a description, and an inputSchema built by schema()
- 'error' is listed as required in the schema; 'budget' is optional
- the default of 1500 for 'budget' is only documented here; it is applied in call

## Terms

tool definition, tool schema, input schema, MCP tool description, required error field, budget parameter, token budget docs

## Connections

- **Location:** `crates/myrn-mcp/src/tools/recall.rs:9-19`
- **Signature:** `pub fn spec() -> Value`
- **In:** [[file-crates-myrn-mcp-src-tools-recall-rs]]
- **Calls:** [[tools-schema]]
- **Called by:** [[tools-list]]
