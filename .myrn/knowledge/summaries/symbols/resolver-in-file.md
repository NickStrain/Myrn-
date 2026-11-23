---
id: resolver-in-file
type: summary
status: unverified
title: Resolver::in_file
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::in_file
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::in_file@05c4180609b87c7f
map_version: '2'
---
# Resolver::in_file

Returns the indexes of symbols in a file path that satisfy a predicate.

## Behaviour

- unknown file -> empty vec
- predicate false for every symbol -> empty vec
- file with matching symbols -> their indexes in index order

## Terms

file symbols, filter by file, symbols in file, predicate filter

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:870-875`
- **Signature:** `fn in_file(&self, file: &str, pred: impl Fn(&Sym) -> bool) -> Vec<usize>`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
- **Called by:** [[resolver-resolve-bare]], [[resolver-exported-from]], [[resolver-resolve-method]]
