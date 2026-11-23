---
id: resolver-lookup
type: summary
status: unverified
title: Resolver::lookup
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::lookup
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::lookup@925cb0ada4d548eb
map_version: '2'
---
# Resolver::lookup

Finds non-test symbols with a qualified name, limited to the same language family as the near file, preferring the same file and then the same Rust crate.

## Behaviour

- no symbol with that name -> empty vec
- one match -> returned even if in another file
- several matches and one in the near file -> only that one is returned

## Terms

qualified name lookup, candidate symbols, same file preference, language family, test exclusion

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:427-457`
- **Signature:** `fn lookup(&self, qname: &str, near: Option<&str>) -> Vec<usize>`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
- **Calls:** [[lang-family]], [[resolver-lookup-in-crate]]
- **Called by:** [[resolver-resolve-bare]], [[resolver-resolve-path]], [[resolver-follow-rust]], [[resolver-reexported]], [[resolver-exported-from]], [[resolver-resolve-method]]
