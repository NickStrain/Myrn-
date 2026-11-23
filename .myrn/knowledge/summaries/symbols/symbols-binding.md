---
id: symbols-binding
type: summary
status: unverified
title: symbols::Binding
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Binding
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Binding@0c776eed7b3692cb
map_version: '2'
---
# symbols::Binding

Describes what a local import name refers to: an optional target file and an optional qualified name of the imported item.

## Behaviour

- file None and qname None -> external import outside the repository
- file set and qname None -> whole-file binding, such as a namespace import
- qname set and file None -> not produced by the current code

## Terms

import binding, local name, alias target, imported item, namespace binding

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:66-70`
- **Signature:** `struct Binding`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
