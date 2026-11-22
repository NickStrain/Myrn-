---
id: file-crates-myrn-store-src-lib-rs
type: summary
status: unverified
title: crates/myrn-store/src/lib.rs
source: myrn summarize
about: file:crates/myrn-store/src/lib.rs
anchors:
- file:crates/myrn-store/src/lib.rs@8a54b69f4d3699f4
---
# crates/myrn-store/src/lib.rs

Crate root for myrn-store, the storage layer for Myrn: it declares the submodules (db, episodes, fts, graph, notes, paths, state, sync, vector) and re-exports Db, Edge, Node, Layout, and State. Per the module doc, Markdown notes are the source of truth for earned knowledge, the SQLite database (code graph and index) is always rebuildable from code and notes, and writes go note first, then index.

## Connections

- **Location:** `crates/myrn-store/src/lib.rs`
- **In:** [[dir-crates-myrn-store-src]]
- **Imports:** [[file-crates-myrn-store-src-db-rs]], [[file-crates-myrn-store-src-episodes-rs]], [[file-crates-myrn-store-src-fts-rs]], [[file-crates-myrn-store-src-graph-rs]], [[file-crates-myrn-store-src-notes-mod-rs]], [[file-crates-myrn-store-src-paths-rs]], [[file-crates-myrn-store-src-state-rs]], [[file-crates-myrn-store-src-sync-rs]], [[file-crates-myrn-store-src-vector-rs]]
- **Imported by:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-engine-src-capture-consent-rs]], [[file-crates-myrn-engine-src-retrieve-rank-rs]], [[file-crates-myrn-engine-src-retrieve-seed-rs]], [[file-crates-myrn-store-src-graph-rs]]
