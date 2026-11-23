---
id: confirm-spec
type: summary
status: unverified
title: confirm::spec
source: myrn summarize
about: sym:crates/myrn-mcp/src/tools/confirm.rs#tools::confirm::spec
anchors:
- sym:crates/myrn-mcp/src/tools/confirm.rs#tools::confirm::spec@906a213c500cae17
map_version: '2'
---
# confirm::spec

Returns the MCP tool definition for 'confirm' as a JSON value, with a description and an input schema that requires a string 'id'. It takes no arguments and reads no engine state.

## Behaviour

- no input -> always the same JSON object with name 'confirm' and inputSchema requiring 'id'
- 'id' is documented as a record id such as 'bug-7f3a91'
- description tells callers to check the code first and to use 'supersede' if the record no longer holds

## Terms

mcp tool schema, tool definition, possibly outdated, stale record, anchors, revalidate record, input schema

## Connections

- **Location:** `crates/myrn-mcp/src/tools/confirm.rs:9-19`
- **Signature:** `pub fn spec() -> Value`
- **In:** [[file-crates-myrn-mcp-src-tools-confirm-rs]]
- **Calls:** [[tools-schema]]
- **Called by:** [[tools-list]]
