---
id: record-call
type: summary
status: unverified
title: record::call
source: myrn summarize
about: sym:crates/myrn-mcp/src/tools/record.rs#tools::record::call
anchors:
- sym:crates/myrn-mcp/src/tools/record.rs#tools::record::call@25d5e23ca898c70f
map_version: '2'
---
# record::call

Stores the record arguments through the engine and returns either the single outcome's text or a count line followed by each item's outcome.

## Behaviour

- one outcome -> that outcome's render() text alone, with no count line
- 3 outcomes, 2 accepted -> "2 of 3 items stored." then a blank line and "Item 1: ..." for each item
- engine error from record_any_json -> propagated via ?, no text returned

## Terms

store records, batch result, record outcome, stored count, item summary, multi-item response

## Connections

- **Location:** `crates/myrn-mcp/src/tools/record.rs:52-63`
- **Signature:** `pub fn call(engine: &mut Engine, args: &Value) -> Result<String>`
- **In:** [[file-crates-myrn-mcp-src-tools-record-rs]]
- **Called by:** [[tools-call]]
