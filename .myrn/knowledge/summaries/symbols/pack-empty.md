---
id: pack-empty
type: summary
status: unverified
title: Pack::empty
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Pack::empty
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Pack::empty@33691fae02bb4aed
map_version: '2'
---
# Pack::empty

Returns a pack with no text, no items and zero tokens.

## Behaviour

- Used when ranking finds nothing relevant for a task

## Terms

empty pack, no context, nothing relevant

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:1301-1307`
- **Signature:** `pub fn empty() -> Pack`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Called by:** [[engine-context-excluding]]
