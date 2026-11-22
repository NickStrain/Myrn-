---
id: impact-spec
type: summary
status: unverified
title: impact::spec
source: myrn summarize
about: sym:crates/myrn-mcp/src/tools/impact.rs#tools::impact::spec
anchors:
- sym:crates/myrn-mcp/src/tools/impact.rs#tools::impact::spec@15ad147cfd4a2cfa
---
# impact::spec

Returns the MCP tool descriptor JSON for `impact`, with a required string input `symbol` accepting a symbol name, qualified name, file path, or path:line. Pure; no side effects.

## Connections

- **Location:** `crates/myrn-mcp/src/tools/impact.rs:9-18`
- **Signature:** `pub fn spec() -> Value`
- **In:** [[file-crates-myrn-mcp-src-tools-impact-rs]]
- **Calls:** [[tools-schema]]
- **Called by:** [[tools-list]]
