---
id: lang-lang
type: summary
status: unverified
title: lang::Lang
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/lang/mod.rs#indexer::lang::Lang
anchors:
- sym:crates/myrn-engine/src/indexer/lang/mod.rs#indexer::lang::Lang@5823b7fbec518f45
map_version: '2'
---
# lang::Lang

Enum of the five supported languages. It is Copy and hashable, and ALL lists the variants in declaration order.

## Behaviour

- Lang::ALL -> [Rust, Python, JavaScript, TypeScript, Tsx]
- JavaScript, TypeScript and Tsx are separate variants with separate grammars

## Terms

language, supported language, file language, language kind, language variant

## Connections

- **Location:** `crates/myrn-engine/src/indexer/lang/mod.rs:8-14`
- **Signature:** `pub enum Lang`
- **In:** [[file-crates-myrn-engine-src-indexer-lang-mod-rs]]
