---
id: resolver-resolved
type: summary
status: unverified
title: Resolver::resolved
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::resolved
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::resolved@804dddda88dae0ab
map_version: '2'
---
# Resolver::resolved

Associated function, no self: marks each target index as Resolved.

## Behaviour

- [3, 5] -> [(3, Resolved), (5, Resolved)]
- empty list -> empty list

## Terms

mark resolved, certain target, exact match, edge certainty

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:487-489`
- **Signature:** `fn resolved(v: Vec<usize>) -> Vec<(usize, EdgeStatus)>`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
- **Called by:** [[resolver-resolve-bare]], [[resolver-resolve-path]], [[resolver-resolve-method]]
