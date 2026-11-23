---
id: resolver-index-of
type: summary
status: unverified
title: Resolver::index_of
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::index_of
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::index_of@b2a271e63feb3135
map_version: '2'
---
# Resolver::index_of

Returns the symbol position for a node id, if the id is known.

## Behaviour

- known id -> Some(position)
- unknown id -> None

## Terms

symbol id, node id lookup, id to index, position lookup

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:877-879`
- **Signature:** `pub fn index_of(&self, id: &str) -> Option<usize>`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
- **Called by:** [[symbols-resolve-all]], [[tests-targets]]
