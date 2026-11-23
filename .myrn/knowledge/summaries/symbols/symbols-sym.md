---
id: symbols-sym
type: summary
status: unverified
title: symbols::Sym
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Sym
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Sym@0a1ef391e729200d
map_version: '2'
---
# symbols::Sym

Holds one symbol or test node as the resolver sees it: id, qualified name, short name, kind, file path, language and is_test flag.

## Behaviour

- qname may end in a '#n' collision suffix; base_qname strips it
- is_test is true only for test nodes; callable() rejects them

## Terms

symbol record, node row, test flag, collision suffix, qualified name

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:22-30`
- **Signature:** `pub struct Sym`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
