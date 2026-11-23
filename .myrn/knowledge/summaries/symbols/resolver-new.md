---
id: resolver-new
type: summary
status: unverified
title: Resolver::new
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::new
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::new@b82656eab46147c8
map_version: '2'
---
# Resolver::new

Builds the name, qualified name, path and id indexes, plus Rust module and crate, Python module and JS file maps. Files are sorted so lib.rs comes before other files, making a crate root resolve to the library.

## Behaviour

- lib.rs and main.rs both at crate root -> the lib.rs path is registered first and wins
- two Python files with the same dotted module name -> the first in sorted order wins
- crates map is stored as given; keys are underscored crate names

## Terms

constructor, index building, crate root, module map, file ordering

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:167-217`
- **Signature:** `pub fn new( syms: Vec<Sym>, files: &[(String, Lang)], crates: HashMap<String, String>, ) -> Resolver`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
- **Calls:** [[lang-family]], [[lang-rust-module]], [[lang-python-module]], [[sym-base-qname]]
- **Called by:** [[symbols-resolve-all]]
- **Tests:** `rust_paths_imports_and_self`, `external_imports_block_guesses`, `python_imports_and_methods`, `js_relative_imports_and_inference`
