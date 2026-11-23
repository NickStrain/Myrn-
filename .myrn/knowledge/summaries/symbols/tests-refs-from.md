---
id: tests-refs-from
type: summary
status: unverified
title: tests::refs_from
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::tests::refs_from
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::tests::refs_from@746f987573f90cd1
map_version: '2'
---
# tests::refs_from

Test helper that returns (kind, name, qualifier) for every Ref whose caller is the named Def.

## Behaviour

- Refs inside a nested def belong to that nested def, not the outer one
- An unknown qname panics through unwrap on the position lookup
- Returns cloned values in the order they were recorded

## Terms

refs from def, callers, call list, test assertion helper, outgoing calls

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:1304-1311`
- **Signature:** `fn refs_from(p: &Parsed, qname: &str) -> Vec<(RefKind, String, Option<String>)>`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Tests:** `rust_defs_refs_imports`, `local_types_turn_method_calls_into_paths`, `rust_calls_inside_macros`, `python_defs_refs_imports`, `typescript_defs_refs_imports_tests`
