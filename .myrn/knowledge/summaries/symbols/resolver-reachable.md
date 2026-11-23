---
id: resolver-reachable
type: summary
status: unverified
title: Resolver::reachable
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::reachable
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::reachable@d12d4c37beaa9751
map_version: '2'
---
# Resolver::reachable

Decides whether code in one file can plausibly reach a symbol in another without naming it: same file, same Rust crate, a direct import, or an import of an import.

## Behaviour

- same path -> true
- two Rust files in the same crate -> true
- a imports b and b imports c -> a reaches c

## Terms

reachability, import graph, two hops, same crate, visibility, can reach

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:828-844`
- **Signature:** `fn reachable(&self, from: &str, to: &str) -> bool`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
- **Calls:** [[lang-from-path]], [[lang-rust-module]]
- **Called by:** [[resolver-resolve-bare]], [[resolver-resolve-path]], [[resolver-resolve-method]]
