---
id: states-is-shared-key
type: summary
status: unverified
title: states::is_shared_key
source: myrn summarize
about: sym:crates/myrn-engine/src/validity/states.rs#validity::states::is_shared_key
anchors:
- sym:crates/myrn-engine/src/validity/states.rs#validity::states::is_shared_key@e4d5659bfce9ce47
map_version: '2'
---
# states::is_shared_key

Reports whether a notes-table key is a file inside the committed knowledge directory.

## Behaviour

- Key starting with REPO_DIR followed by /knowledge/ -> true
- Any other key -> false

## Terms

shared record, committed knowledge, knowledge file, shared note

## Connections

- **Location:** `crates/myrn-engine/src/validity/states.rs:357-359`
- **Signature:** `pub(crate) fn is_shared_key(key: &str) -> bool`
- **In:** [[file-crates-myrn-engine-src-validity-states-rs]]
- **Called by:** [[engine-publish-plan]], [[engine-promote]]
