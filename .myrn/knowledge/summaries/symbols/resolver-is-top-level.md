---
id: resolver-is-top-level
type: summary
status: unverified
title: Resolver::is_top_level
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::is_top_level
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::is_top_level@16a73add0b0c28b8
map_version: '2'
---
# Resolver::is_top_level

Reports whether a symbol sits directly at module scope of its file, using the language's own module naming.

## Behaviour

- JS symbol with empty parent -> true
- Python method whose parent is a class -> false
- Rust item whose parent equals the file's module path -> true

## Terms

top-level, module scope, module level, file scope, direct member

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:862-868`
- **Signature:** `fn is_top_level(&self, t: &Sym) -> bool`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
- **Calls:** [[lang-family]], [[lang-rust-module]], [[lang-python-module]], [[sym-parent]]
- **Called by:** [[resolver-exported-from]]
