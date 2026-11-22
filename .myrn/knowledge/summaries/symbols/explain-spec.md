---
id: explain-spec
type: summary
status: unverified
title: explain::spec
source: myrn summarize
about: sym:crates/myrn-mcp/src/tools/explain.rs#tools::explain::spec
anchors:
- sym:crates/myrn-mcp/src/tools/explain.rs#tools::explain::spec@e538b4353e9ed435
---
# explain::spec

Returns the MCP tool JSON for "explain": a description of the card contents and an inputSchema with one required string property "symbol" that accepts a name, qualified name (a::b or A.b), path, or path:line.

## Connections

- **Location:** `crates/myrn-mcp/src/tools/explain.rs:9-18`
- **Signature:** `pub fn spec() -> Value`
- **In:** [[file-crates-myrn-mcp-src-tools-explain-rs]]
- **Calls:** [[tools-schema]]
- **Called by:** [[tools-list]]
