---
id: parse-scopekind
type: summary
status: unverified
title: parse::ScopeKind
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::ScopeKind
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::ScopeKind@90d5de3b9dafc62f
map_version: '2'
---
# parse::ScopeKind

Marks what kind of container a scope stack entry is, which decides method naming and module paths.

## Behaviour

- Type (impl, class, trait, interface): functions directly inside become methods
- Module (inline mod, namespace, describe block): adds a segment to the Rust module path or the qualified name
- Func: a function scope; its name still joins the qualified names of nested items but does not make them methods

## Terms

scope kind, container, nesting, scope marker, block kind, impl block

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:134-140`
- **Signature:** `enum ScopeKind`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
