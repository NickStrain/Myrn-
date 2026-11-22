---
id: file-crates-myrn-engine-src-indexer-mod-rs
type: summary
status: unverified
title: crates/myrn-engine/src/indexer/mod.rs
source: myrn summarize
about: file:crates/myrn-engine/src/indexer/mod.rs
anchors:
- file:crates/myrn-engine/src/indexer/mod.rs@cf2aafb82506fbd9
---
# crates/myrn-engine/src/indexer/mod.rs

Builds the derived code and Git-history graph from the working tree. It exposes refresh, which re-parses only files whose content changed, plus helpers for parsing, persisting, deleting, and garbage-collecting module nodes.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/mod.rs`
- **In:** [[dir-crates-myrn-engine-src-indexer]]
- **Defines:** [[indexer-indexreport]], [[indexreport-changed]], [[indexer-content-hash]], [[indexer-refresh]], [[indexer-parseresult]], [[indexer-parse-all]], [[indexer-delete-file]], [[indexer-persist]], [[indexer-gc-modules]]
- **Imports:** [[file-crates-myrn-engine-src-config-rs]], [[file-crates-myrn-engine-src-indexer-body-hash-rs]], [[file-crates-myrn-engine-src-indexer-git-rs]], [[file-crates-myrn-engine-src-indexer-lang-mod-rs]], [[file-crates-myrn-engine-src-indexer-parse-rs]], [[file-crates-myrn-engine-src-indexer-symbols-rs]], [[file-crates-myrn-engine-src-indexer-walk-rs]], [[file-crates-myrn-model-src-id-rs]], [[file-crates-myrn-store-src-fts-rs]], [[file-crates-myrn-store-src-graph-rs]]
- **Imported by:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-engine-src-lib-rs]]
