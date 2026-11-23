---
id: resolver-lookup-in-crate
type: summary
status: unverified
title: Resolver::lookup_in_crate
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::lookup_in_crate
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::lookup_in_crate@e7449f275f1578b8
map_version: '2'
---
# Resolver::lookup_in_crate

Returns the first non-test Rust symbol with the qualified name that is in the same crate as the given file.

## Behaviour

- match only in another crate -> None
- non-Rust symbol with the same name -> skipped
- several matches in the crate -> the first in index order

## Terms

crate lookup, same crate, Rust crate match, crate scope

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:459-465`
- **Signature:** `fn lookup_in_crate(&self, qname: &str, near: &str) -> Option<usize>`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
- **Calls:** [[lang-family]], [[lang-rust-module]]
- **Called by:** [[resolver-rust-path]], [[resolver-lookup]]
