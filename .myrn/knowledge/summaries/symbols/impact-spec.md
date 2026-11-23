---
id: impact-spec
type: summary
status: unverified
title: impact::spec
source: myrn summarize
about: sym:crates/myrn-mcp/src/tools/impact.rs#tools::impact::spec
anchors:
- sym:crates/myrn-mcp/src/tools/impact.rs#tools::impact::spec@15ad147cfd4a2cfa
map_version: '2'
---
# impact::spec

Builds the JSON tool definition for 'impact', with a description and an input schema that requires a string 'symbol'.

## Behaviour

- no input -> always returns the same fixed JSON object with name 'impact'
- 'symbol' accepts a symbol name, qualified name, file path, or path:line
- 'symbol' is the only property and is listed as required

## Terms

tool schema, tool definition, input schema, MCP tool metadata, advertised tool, tool registration

## Connections

- **Location:** `crates/myrn-mcp/src/tools/impact.rs:9-18`
- **Signature:** `pub fn spec() -> Value`
- **In:** [[file-crates-myrn-mcp-src-tools-impact-rs]]
- **Calls:** [[tools-schema]]
- **Called by:** [[tools-list]]
