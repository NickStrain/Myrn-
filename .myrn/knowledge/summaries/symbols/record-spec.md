---
id: record-spec
type: summary
status: unverified
title: record::spec
source: myrn summarize
about: sym:crates/myrn-mcp/src/tools/record.rs#tools::record::spec
anchors:
- sym:crates/myrn-mcp/src/tools/record.rs#tools::record::spec@f606d79d4e1992cf
map_version: '2'
---
# record::spec

Builds and returns the JSON tool descriptor for `record`, with its description text and input schema covering single-record fields and an `items` array for batches.

## Behaviour

- no args -> Value object with name "record", a description string, and inputSchema
- type field is an enum of bug, decision, workflow, fact, summary
- about and verified_by accept a string or an array of strings; evidence is an array of objects with string values

## Terms

tool schema, input schema, tool definition, tool description, field list, MCP tool spec

## Connections

- **Location:** `crates/myrn-mcp/src/tools/record.rs:9-50`
- **Signature:** `pub fn spec() -> Value`
- **In:** [[file-crates-myrn-mcp-src-tools-record-rs]]
- **Calls:** [[tools-schema]]
- **Called by:** [[tools-list]]
