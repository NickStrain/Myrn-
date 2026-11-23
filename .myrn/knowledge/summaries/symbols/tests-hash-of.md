---
id: tests-hash-of
type: summary
status: unverified
title: tests::hash_of
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/body_hash.rs#indexer::body_hash::tests::hash_of
anchors:
- sym:crates/myrn-engine/src/indexer/body_hash.rs#indexer::body_hash::tests::hash_of@d24fe65a56d6c694
map_version: '2'
---
# tests::hash_of

Test helper that parses the given Rust source as src/lib.rs and returns the body_hash of the first parsed definition, panicking if parsing fails or no definition exists.

## Behaviour

- "fn f(x: i32) -> i32 { x + 1 }" -> body_hash of defs[0]
- a source with no definitions, or one that fails to parse, panics via unwrap or index access
- the file path is always "src/lib.rs" and the language is always Lang::Rust

## Terms

test helper, parse first definition, body hash in tests, fixture parser

## Connections

- **Location:** `crates/myrn-engine/src/indexer/body_hash.rs:38-43`
- **Signature:** `fn hash_of(src: &str) -> String`
- **In:** [[file-crates-myrn-engine-src-indexer-body-hash-rs]]
- **Calls:** [[fileparser-new]], [[fileparser-parse]]
- **Tests:** `ignores_formatting_and_comments`
