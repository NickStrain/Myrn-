---
id: file-crates-myrn-store-src-fts-rs
type: summary
status: unverified
title: crates/myrn-store/src/fts.rs
source: myrn summarize
about: file:crates/myrn-store/src/fts.rs
anchors:
- file:crates/myrn-store/src/fts.rs@e3dfe0453f09dcee
---
# crates/myrn-store/src/fts.rs

SQLite FTS5 search layer for the code map. It writes and deletes rows in the `search` table keyed by nodes.rowid, runs ranked queries over `search` (name, terms, path, doc) and over `summary_search` (code-map notes), and includes the identifier splitter used to build indexed terms.

## Connections

- **Location:** `crates/myrn-store/src/fts.rs`
- **In:** [[dir-crates-myrn-store-src]]
- **Defines:** [[fts-split-identifier]], [[fts-index-node]], [[fts-remove-path]], [[fts-query]], [[fts-query-summaries]], [[fts-summary-details]]
- **Imported by:** [[file-crates-myrn-engine-src-embeddings-rs]], [[file-crates-myrn-engine-src-indexer-mod-rs]], [[file-crates-myrn-engine-src-retrieve-seed-rs]], [[file-crates-myrn-store-src-lib-rs]], [[file-crates-myrn-store-src-sync-rs]]
