---
id: lang-family-mod
type: summary
status: unverified
title: lang::Family
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/lang/mod.rs#indexer::lang::Family
anchors:
- sym:crates/myrn-engine/src/indexer/lang/mod.rs#indexer::lang::Family@2cb4c94d8d5ae46d
map_version: '2'
---
# lang::Family

Three resolution families (Rust, Python, Js) that group languages sharing naming and resolution rules.

## Behaviour

- Family::Js covers JavaScript, TypeScript and Tsx
- Family::Rust covers only Rust
- Family::Python covers only Python

## Terms

family, language group, resolution rules, shared rules, group

## Connections

- **Location:** `crates/myrn-engine/src/indexer/lang/mod.rs:84-88`
- **Signature:** `pub enum Family`
- **In:** [[file-crates-myrn-engine-src-indexer-lang-mod-rs]]
