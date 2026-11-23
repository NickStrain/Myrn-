---
id: adapter-str-field
type: summary
status: unverified
title: adapter::str_field
source: myrn summarize
about: sym:crates/myrn-adapters/src/adapter.rs#adapter::str_field
anchors:
- sym:crates/myrn-adapters/src/adapter.rs#adapter::str_field@649387bfd7662d6b
map_version: '2'
---
# adapter::str_field

Reads a string field from a JSON value and returns it as an owned String.

## Behaviour

- {"key": "x"} with key 'key' -> Some('x')
- Missing key or non-string value (number, null) -> None
- v that is not an object -> None

## Terms

json string, get field, read key, string value

## Connections

- **Location:** `crates/myrn-adapters/src/adapter.rs:95-97`
- **Signature:** `pub(crate) fn str_field(v: &Value, key: &str) -> Option<String>`
- **In:** [[file-crates-myrn-adapters-src-adapter-rs]]
- **Called by:** [[claudecode-translate]], [[codex-failure-text]], [[codex-translate]]
