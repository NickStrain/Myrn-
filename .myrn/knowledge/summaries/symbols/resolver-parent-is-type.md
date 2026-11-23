---
id: resolver-parent-is-type
type: summary
status: unverified
title: Resolver::parent_is_type
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::parent_is_type
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::parent_is_type@8272b1d3e01634ed
map_version: '2'
---
# Resolver::parent_is_type

Reports whether a method's parent is a class, interface, struct, enum or trait defined in the repository. Rust always returns true.

## Behaviour

- Rust symbol -> true regardless of parent
- JS method inside an object literal in a function -> false
- parent qname matching a class-kind symbol -> true

## Terms

type parent, owning type, class member, object literal, impl block

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:848-860`
- **Signature:** `fn parent_is_type(&self, t: &Sym) -> bool`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
- **Calls:** [[kind-as-str]], [[lang-family]], [[sym-parent]]
- **Called by:** [[resolver-resolve-method]]
