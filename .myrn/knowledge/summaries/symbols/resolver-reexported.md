---
id: resolver-reexported
type: summary
status: unverified
title: Resolver::reexported
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::reexported
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::reexported@a1b577609f135579
map_version: '2'
---
# Resolver::reexported

Finds what a name in a file refers to after an import, using the bound target if present and otherwise exported_from with depth 3.

## Behaviour

- name bound with a qname -> lookup of that qname in the bound file
- no binding -> exported_from with depth 3
- binding without a qname -> exported_from fallback

## Terms

re-export, bound name, exported name, import target, forwarded name

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:674-682`
- **Signature:** `fn reexported(&self, file: &str, name: &str) -> Vec<usize>`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
- **Calls:** [[resolver-lookup]], [[resolver-binding]], [[resolver-exported-from]]
- **Called by:** [[resolver-resolve-bare]]
