---
id: symbols-resolver
type: summary
status: unverified
title: symbols::Resolver
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver@4498f2bbe4514601
map_version: '2'
---
# symbols::Resolver

Holds all symbols, the lookup indexes, Rust/Python/JS module maps, per-file import bindings, wildcard imports and file import graph that resolution uses. Built once per resolve_all pass.

## Behaviour

- state is filled by new, then add_import records imports
- resolve reads this state and does not change it
- indexes map names, qualified names, paths and ids to symbol positions

## Terms

resolver state, lookup tables, import graph, symbol index, bindings

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:148-163`
- **Signature:** `pub struct Resolver`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
