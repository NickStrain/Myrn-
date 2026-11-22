---
id: resolver-reexported
type: summary
status: unverified
title: Resolver::reexported
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::reexported
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::reexported@a1b577609f135579
---
# Resolver::reexported

Resolves a name exported from a file by following that file's use binding for it, or falling back to exported_from.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:674-682`
- **Signature:** `fn reexported(&self, file: &str, name: &str) -> Vec<usize>`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
- **Calls:** [[resolver-lookup]], [[resolver-binding]], [[resolver-exported-from]]
- **Called by:** [[resolver-resolve-bare]]
