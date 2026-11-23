---
id: tools-schema
type: summary
status: unverified
title: tools::schema
source: myrn summarize
about: sym:crates/myrn-mcp/src/tools/mod.rs#tools::schema
anchors:
- sym:crates/myrn-mcp/src/tools/mod.rs#tools::schema@46a5e63ec81494e8
map_version: '2'
---
# tools::schema

Builds a JSON Schema object with type object, the given properties, and the required list. It does no validation of its inputs.

## Behaviour

- props {"a":{...}} with required ["a"] -> object schema with properties and required set
- empty required slice -> "required": [] is still emitted
- a required name missing from props is not checked and passes through unchanged

## Terms

JSON schema, input schema, tool parameters, object schema, required fields, argument spec

## Connections

- **Location:** `crates/myrn-mcp/src/tools/mod.rs:109-111`
- **Signature:** `pub(crate) fn schema(props: Value, required: &[&str]) -> Value`
- **In:** [[file-crates-myrn-mcp-src-tools-mod-rs]]
- **Called by:** [[confirm-spec]], [[context-spec]], [[explain-spec]], [[history-spec]], [[impact-spec]], [[recall-spec]], [[record-spec]], [[supersede-spec]]
