---
id: tests-files
type: summary
status: unverified
title: tests::files
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::tests::files
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::tests::files@a6e027b9b93cec92
map_version: '2'
---
# tests::files

Test helper that builds a (path, language) list from symbols, removing only consecutive duplicates.

## Behaviour

- symbols from one file listed together -> one entry
- same path appearing non-consecutively -> duplicate entries kept
- empty symbol list -> empty list

## Terms

test helper, file list, fixture files, dedup, path and language

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:1088-1092`
- **Signature:** `fn files(syms: &[Sym]) -> Vec<(String, Lang)>`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
- **Tests:** `rust_paths_imports_and_self`, `external_imports_block_guesses`, `python_imports_and_methods`, `js_relative_imports_and_inference`
