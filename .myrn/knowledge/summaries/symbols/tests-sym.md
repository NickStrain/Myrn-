---
id: tests-sym
type: summary
status: unverified
title: tests::sym
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::tests::sym
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::tests::sym@2df0f596152b8529
map_version: '2'
---
# tests::sym

Test helper that builds a Sym from a path, qualified name and kind, deriving language, short name, id and is_test.

## Behaviour

- ('src/a.rs', 'a::f', 'function') -> name 'f'
- kind 'test' -> is_test true
- path with unsupported extension -> panics on unwrap

## Terms

test helper, fixture symbol, make symbol, sample node

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:1051-1067`
- **Signature:** `fn sym(path: &str, qname: &str, kind: &str) -> Sym`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
- **Calls:** [[lang-from-path]], [[symbols-strip-suffix]], [[id-symbol-id]]
- **Tests:** `rust_paths_imports_and_self`, `external_imports_block_guesses`, `python_imports_and_methods`, `js_relative_imports_and_inference`
