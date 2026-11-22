---
id: tests-sym
type: summary
status: unverified
title: tests::sym
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::tests::sym
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::tests::sym@2df0f596152b8529
---
# tests::sym

Test helper that builds a Sym from a path and qname, deriving name from the last segment and is_test from the kind.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:1051-1067`
- **Signature:** `fn sym(path: &str, qname: &str, kind: &str) -> Sym`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
- **Calls:** [[lang-from-path]], [[symbols-strip-suffix]], [[id-symbol-id]]
- **Tests:** `rust_paths_imports_and_self`, `external_imports_block_guesses`, `python_imports_and_methods`, `js_relative_imports_and_inference`
