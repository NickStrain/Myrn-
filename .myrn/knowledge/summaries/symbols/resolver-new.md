---
id: resolver-new
type: summary
status: unverified
title: Resolver::new
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::new
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::new@b82656eab46147c8
---
# Resolver::new

Builds the indexes from the symbols and a (path, lang) file list; crates maps underscored Rust crate names to directories. lib.rs sorts first so it wins Rust module-path collisions, and the first file wins for other duplicates.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:167-217`
- **Signature:** `pub fn new( syms: Vec<Sym>, files: &[(String, Lang)], crates: HashMap<String, String>, ) -> Resolver`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
- **Calls:** [[lang-family]], [[lang-rust-module]], [[lang-python-module]], [[sym-base-qname]]
- **Called by:** [[symbols-resolve-all]]
- **Tests:** `rust_paths_imports_and_self`, `external_imports_block_guesses`, `python_imports_and_methods`, `js_relative_imports_and_inference`
