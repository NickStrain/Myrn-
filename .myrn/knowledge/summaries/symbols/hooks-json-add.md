---
id: hooks-json-add
type: summary
status: unverified
title: hooks_json::add
source: myrn summarize
about: sym:crates/myrn-adapters/src/adapter.rs#adapter::hooks_json::add
anchors:
- sym:crates/myrn-adapters/src/adapter.rs#adapter::hooks_json::add@0cf905a086042daf
map_version: '2'
---
# hooks_json::add

Appends a new group containing the handler under doc.hooks[event], creating the hooks object and event array when absent.

## Behaviour

- Non-object doc -> replaced by an empty object first
- Event already holds an array -> new group pushed, existing groups untouched
- Event value not an array -> nothing added, no error; hooks value not an object -> panic

## Terms

insert hook, append handler, new hook group, register hook

## Connections

- **Location:** `crates/myrn-adapters/src/adapter.rs:166-183`
- **Signature:** `pub fn add(doc: &mut Value, event: &str, handler: Value)`
- **In:** [[file-crates-myrn-adapters-src-adapter-rs]]
- **Called by:** [[claudecode-install]], [[codex-install]]
