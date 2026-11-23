---
id: tests-call
type: summary
status: unverified
title: tests::call
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::tests::call
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::tests::call@9d09124b4a3c6925
map_version: '2'
---
# tests::call

Test helper that builds a RawRef from a source symbol with a kind, name, optional qualifier and optional scope.

## Behaviour

- qualifier Some('Self') -> stored as qualifier
- None qualifier -> None stored
- path copied from the source symbol

## Terms

test helper, fixture reference, raw ref builder, sample call

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:1069-1078`
- **Signature:** `fn call(src: &Sym, kind: &str, name: &str, q: Option<&str>, scope: Option<&str>) -> RawRef`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
- **Tests:** `rust_paths_imports_and_self`, `external_imports_block_guesses`, `python_imports_and_methods`, `js_relative_imports_and_inference`
