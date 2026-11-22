---
id: record-call
type: summary
status: unverified
title: record::call
source: myrn summarize
about: sym:crates/myrn-mcp/src/tools/record.rs#tools::record::call
anchors:
- sym:crates/myrn-mcp/src/tools/record.rs#tools::record::call@25d5e23ca898c70f
---
# record::call

Passes `args` to `engine.record_any_json` and returns the rendered outcome for a single item. For multiple items it returns "N of M items stored." followed by each item's rendering, numbered from 1. Errors from the engine are propagated with `?`.

## Connections

- **Location:** `crates/myrn-mcp/src/tools/record.rs:52-63`
- **Signature:** `pub fn call(engine: &mut Engine, args: &Value) -> Result<String>`
- **In:** [[file-crates-myrn-mcp-src-tools-record-rs]]
- **Called by:** [[tools-call]]
