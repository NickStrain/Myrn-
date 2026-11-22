---
id: tests-hash-of
type: summary
status: unverified
title: tests::hash_of
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/body_hash.rs#indexer::body_hash::tests::hash_of
anchors:
- sym:crates/myrn-engine/src/indexer/body_hash.rs#indexer::body_hash::tests::hash_of@d24fe65a56d6c694
---
# tests::hash_of

Parses the given Rust source as src/lib.rs with FileParser and returns the body_hash of the first definition in the result. It panics if parsing fails or if the source contains no definitions.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/body_hash.rs:38-43`
- **Signature:** `fn hash_of(src: &str) -> String`
- **In:** [[file-crates-myrn-engine-src-indexer-body-hash-rs]]
- **Calls:** [[fileparser-new]], [[fileparser-parse]]
- **Tests:** `ignores_formatting_and_comments`
