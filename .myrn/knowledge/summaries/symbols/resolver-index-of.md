---
id: resolver-index-of
type: summary
status: unverified
title: Resolver::index_of
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::index_of
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::index_of@b2a271e63feb3135
---
# Resolver::index_of

Returns the Resolver index for a symbol id, or None if the id is unknown.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:877-879`
- **Signature:** `pub fn index_of(&self, id: &str) -> Option<usize>`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
- **Called by:** [[symbols-resolve-all]], [[tests-targets]]
