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

Computes a body hash for a tree-sitter node that ignores whitespace and comments, so formatting, comment and line-ending changes keep the same hash while code changes alter it. Its test module checks those rules using a parse helper, hash_of.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/body_hash.rs`
- **In:** [[dir-crates-myrn-engine-src-indexer]]
- **Defines:** [[body-hash-body-hash]], [[tests-hash-of]]
- **Imports:** [[file-crates-myrn-engine-src-indexer-lang-mod-rs]], [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Imported by:** [[file-crates-myrn-engine-src-indexer-mod-rs]], [[file-crates-myrn-engine-src-indexer-parse-rs]]
