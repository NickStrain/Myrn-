---
id: resolver-sym
type: summary
status: unverified
title: Resolver::sym
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::sym
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::sym@7e6131f05c2e6593
map_version: '2'
---
# Resolver::sym

Returns the symbol stored at the given position in the symbols list.

## Behaviour

- position from index_of -> that Sym
- position past the end -> panics from indexing

## Terms

symbol lookup, get symbol, position access, symbol by index

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:219-221`
- **Signature:** `pub fn sym(&self, i: usize) -> &Sym`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
- **Called by:** [[symbols-resolve-all]], [[tests-targets]]
