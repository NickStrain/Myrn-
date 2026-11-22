---
id: hooks-json-add
type: summary
status: unverified
title: hooks_json::add
source: myrn summarize
about: sym:crates/myrn-adapters/src/adapter.rs#adapter::hooks_json::add
anchors:
- sym:crates/myrn-adapters/src/adapter.rs#adapter::hooks_json::add@0cf905a086042daf
---
# hooks_json::add

Appends a new group containing the handler under hooks[event], creating missing objects and arrays. Resets doc to {} if it is not an object, and does not check for duplicates. It panics if hooks is a non-object value, and silently does nothing if hooks[event] exists but is not an array.

## Connections

- **Location:** `crates/myrn-adapters/src/adapter.rs:166-183`
- **Signature:** `pub fn add(doc: &mut Value, event: &str, handler: Value)`
- **In:** [[file-crates-myrn-adapters-src-adapter-rs]]
- **Called by:** [[claudecode-install]], [[codex-install]]
