---
id: tools-schema
type: summary
status: unverified
title: tools::schema
source: myrn summarize
about: sym:crates/myrn-mcp/src/tools/mod.rs#tools::schema
anchors:
- sym:crates/myrn-mcp/src/tools/mod.rs#tools::schema@46a5e63ec81494e8
---
# tools::schema

Builds a JSON Schema object with type object, the given props as properties, and the required key list. It does not validate its inputs.

## Connections

- **Location:** `crates/myrn-mcp/src/tools/mod.rs:57-59`
- **Signature:** `pub(crate) fn schema(props: Value, required: &[&str]) -> Value`
- **In:** [[file-crates-myrn-mcp-src-tools-mod-rs]]
- **Called by:** [[context-spec]], [[explain-spec]], [[history-spec]], [[impact-spec]], [[recall-spec]], [[record-spec]]
