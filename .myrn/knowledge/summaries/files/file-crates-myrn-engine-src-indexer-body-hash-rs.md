---
id: file-crates-myrn-engine-src-indexer-body-hash-rs
type: summary
status: unverified
title: crates/myrn-engine/src/indexer/body_hash.rs
source: myrn summarize
about: file:crates/myrn-engine/src/indexer/body_hash.rs
anchors:
- file:crates/myrn-engine/src/indexer/body_hash.rs@100cac01f30bb38f
---
# crates/myrn-engine/src/indexer/body_hash.rs

Computes a content hash of a tree-sitter syntax node that ignores whitespace and comments, so reformatting, comment edits, and CRLF/LF differences leave the hash unchanged. It holds the hash function and a single unit test that checks those invariants and that a real code change alters the hash.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/body_hash.rs`
- **In:** [[dir-crates-myrn-engine-src-indexer]]
- **Defines:** [[body-hash-body-hash]], [[tests-hash-of]]
- **Imports:** [[file-crates-myrn-engine-src-indexer-lang-mod-rs]], [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Imported by:** [[file-crates-myrn-engine-src-indexer-mod-rs]], [[file-crates-myrn-engine-src-indexer-parse-rs]]
