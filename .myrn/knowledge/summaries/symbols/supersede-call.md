---
id: supersede-call
type: summary
status: unverified
title: supersede::call
source: myrn summarize
about: sym:crates/myrn-mcp/src/tools/supersede.rs#tools::supersede::call
anchors:
- sym:crates/myrn-mcp/src/tools/supersede.rs#tools::supersede::call@733fc4a2755c357c
map_version: '2'
---
# supersede::call

Validates `id` (string) and `item` (must be a JSON object), then calls `engine.supersede(id, item)` and returns the rendered result string. Errors propagate via anyhow.

## Behaviour

- item missing or non-object (string, array, null) -> error "missing required argument `item` (an object)"
- id missing or not a string -> error from `str_arg`
- valid id and object item -> engine replaces the record and the returned value is `.render()` output

## Terms

replace record, supersede handler, correction, outdated record, record update, validate item

## Connections

- **Location:** `crates/myrn-mcp/src/tools/supersede.rs:22-29`
- **Signature:** `pub fn call(engine: &mut Engine, args: &Value) -> Result<String>`
- **In:** [[file-crates-myrn-mcp-src-tools-supersede-rs]]
- **Calls:** [[tools-str-arg]]
- **Called by:** [[tools-call]]
