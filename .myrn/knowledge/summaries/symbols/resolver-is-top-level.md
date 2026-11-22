---
id: resolver-is-top-level
type: summary
status: unverified
title: Resolver::is_top_level
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::is_top_level
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::is_top_level@16a73add0b0c28b8
---
# Resolver::is_top_level

True when a symbol sits directly at its file's module level: an empty parent for JS, or a parent equal to the module path for Python and Rust.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:862-868`
- **Signature:** `fn is_top_level(&self, t: &Sym) -> bool`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
- **Calls:** [[lang-family]], [[lang-rust-module]], [[lang-python-module]], [[sym-parent]]
- **Called by:** [[resolver-exported-from]]
