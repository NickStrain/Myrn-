---
id: adapter-str-field
type: summary
status: unverified
title: adapter::str_field
source: myrn summarize
about: sym:crates/myrn-adapters/src/adapter.rs#adapter::str_field
anchors:
- sym:crates/myrn-adapters/src/adapter.rs#adapter::str_field@649387bfd7662d6b
---
# adapter::str_field

Returns the string value at key in a JSON object, or None if the key is missing or not a string.

## Connections

- **Location:** `crates/myrn-adapters/src/adapter.rs:95-97`
- **Signature:** `pub(crate) fn str_field(v: &Value, key: &str) -> Option<String>`
- **In:** [[file-crates-myrn-adapters-src-adapter-rs]]
- **Called by:** [[claudecode-translate]], [[codex-failure-text]], [[codex-translate]]
