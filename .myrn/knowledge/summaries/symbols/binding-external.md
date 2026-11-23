---
id: binding-external
type: summary
status: unverified
title: Binding::external
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Binding::external
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Binding::external@f2128a5a9d6ffda8
map_version: '2'
---
# Binding::external

Returns true when both file and qname are absent, meaning the import points outside the repository.

## Behaviour

- file None and qname None -> true
- file Some and qname None -> false
- file None and qname Some -> false

## Terms

external import, outside repository, unresolved import, third-party import, unknown target

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:73-75`
- **Signature:** `fn external(&self) -> bool`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
- **Called by:** [[resolver-resolve-bare]], [[resolver-resolve-path]]
