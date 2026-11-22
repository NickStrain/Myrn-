---
id: recall-spec
type: summary
status: unverified
title: recall::spec
source: myrn summarize
about: sym:crates/myrn-mcp/src/tools/recall.rs#tools::recall::spec
anchors:
- sym:crates/myrn-mcp/src/tools/recall.rs#tools::recall::spec@19f242e759a9802b
---
# recall::spec

Returns the JSON tool descriptor for `recall`: name, description, and an input schema requiring the string `error` and accepting an optional integer `budget`.

## Connections

- **Location:** `crates/myrn-mcp/src/tools/recall.rs:9-19`
- **Signature:** `pub fn spec() -> Value`
- **In:** [[file-crates-myrn-mcp-src-tools-recall-rs]]
- **Calls:** [[tools-schema]]
- **Called by:** [[tools-list]]
