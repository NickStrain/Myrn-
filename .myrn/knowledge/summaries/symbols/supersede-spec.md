---
id: supersede-spec
type: summary
status: unverified
title: supersede::spec
source: myrn summarize
about: sym:crates/myrn-mcp/src/tools/supersede.rs#tools::supersede::spec
anchors:
- sym:crates/myrn-mcp/src/tools/supersede.rs#tools::supersede::spec@1205aca47bb704e1
map_version: '2'
---
# supersede::spec

Returns the MCP tool descriptor JSON for `supersede`, with required string `id` and object `item` arguments. The description states that the old record is kept in history, marked superseded, and no longer shown.

## Behaviour

- no args -> JSON object with name "supersede", a description, and inputSchema requiring ["id", "item"]
- item is documented as taking the same fields as `record` (type, body, about, evidence)
- the old record is not deleted; it is kept in history and hidden from display

## Terms

replace record, correct record, outdated record, history, superseded, record correction

## Connections

- **Location:** `crates/myrn-mcp/src/tools/supersede.rs:9-20`
- **Signature:** `pub fn spec() -> Value`
- **In:** [[file-crates-myrn-mcp-src-tools-supersede-rs]]
- **Calls:** [[tools-schema]]
- **Called by:** [[tools-list]]
