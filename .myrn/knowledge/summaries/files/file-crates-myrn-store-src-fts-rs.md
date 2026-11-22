---
id: file-crates-myrn-store-src-fts-rs
type: summary
status: unverified
title: crates/myrn-store/src/fts.rs
source: myrn summarize
about: file:crates/myrn-store/src/fts.rs
anchors:
- file:crates/myrn-store/src/fts.rs@61d53b72e709e608
---
# crates/myrn-store/src/fts.rs

SQLite FTS index over node names, split qualified-name terms, file paths and docs, keyed by nodes.rowid so entries can be removed by path. It holds the identifier splitter, the insert and delete helpers, and two bm25-ranked query functions, one for nodes and one for code summaries.

## Connections

- **Location:** `crates/myrn-store/src/fts.rs`
- **In:** [[dir-crates-myrn-store-src]]
- **Defines:** [[fts-split-identifier]], [[fts-index-node]], [[fts-remove-path]], [[fts-query]], [[fts-query-summaries]]
- **Imported by:** [[file-crates-myrn-engine-src-indexer-mod-rs]], [[file-crates-myrn-engine-src-retrieve-seed-rs]], [[file-crates-myrn-store-src-lib-rs]], [[file-crates-myrn-store-src-sync-rs]]
