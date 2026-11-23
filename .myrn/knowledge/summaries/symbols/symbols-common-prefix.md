---
id: symbols-common-prefix
type: summary
status: unverified
title: symbols::common_prefix
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::common_prefix
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::common_prefix@8e8b846487355d58
map_version: '2'
---
# symbols::common_prefix

Counts how many leading characters two strings share.

## Behaviour

- 'abc' and 'abd' -> 2
- empty and any string -> 0
- identical strings -> their length in characters

## Terms

shared prefix, longest common prefix, closeness, character overlap, nearest parent

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:886-888`
- **Signature:** `fn common_prefix(a: &str, b: &str) -> usize`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
- **Called by:** [[resolver-resolve-bare]]
