---
id: tests-parse
type: summary
status: unverified
title: tests::parse
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::tests::parse
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::tests::parse@1cc98cdd5e1090fd
map_version: '2'
---
# tests::parse

Test helper that parses a source string with a fresh FileParser and panics on error.

## Behaviour

- Takes the language, a fake path and the source text
- The path matters: it sets the Rust module prefix and Python test-file detection
- Returns a Parsed; any Err becomes a panic through unwrap

## Terms

test helper, fixture parse, parse fixture, unwrap, test input

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:1291-1293`
- **Signature:** `fn parse(lang: Lang, path: &str, src: &str) -> Parsed`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Calls:** [[fileparser-new]], [[fileparser-parse]]
- **Tests:** `rust_defs_refs_imports`, `local_types_turn_method_calls_into_paths`, `rust_calls_inside_macros`, `python_defs_refs_imports`, `typescript_defs_refs_imports_tests`
