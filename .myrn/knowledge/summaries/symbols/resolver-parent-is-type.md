---
id: resolver-parent-is-type
type: summary
status: unverified
title: Resolver::parent_is_type
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::parent_is_type
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::parent_is_type@8272b1d3e01634ed
---
# Resolver::parent_is_type

True when a method's parent is a class, interface, struct, enum or trait; always true for Rust impl blocks, and false for methods of object literals.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:848-860`
- **Signature:** `fn parent_is_type(&self, t: &Sym) -> bool`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
- **Calls:** [[kind-as-str]], [[lang-family]], [[sym-parent]]
- **Called by:** [[resolver-resolve-method]]
