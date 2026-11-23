---
id: dir-crates-myrn-store-src
type: summary
status: unverified
title: crates/myrn-store/src/
source: myrn summarize
about: module:crates/myrn-store/src
anchors:
- module:crates/myrn-store/src@544893e76b93ff18
---
# crates/myrn-store/src/

Storage layer for Myrn, where Markdown notes in notes/ are the source of truth for earned knowledge and the SQLite memory.db (code graph, FTS search index) is always rebuildable from code and notes, with writes going note first, then index. db.rs opens the database and runs migrations, graph.rs and fts.rs manage nodes, edges and search rows, sync.rs keeps notes and graph in step, episodes.rs writes append-only JSONL logs, state.rs holds per-project state outside the repo, and paths.rs defines where all of it lives.

## Connections

- **In:** [[dir-crates-myrn-store]]
- **Directories:** [[dir-crates-myrn-store-src-notes]]
- **Files:** [[file-crates-myrn-store-src-db-rs]], [[file-crates-myrn-store-src-episodes-rs]], [[file-crates-myrn-store-src-fts-rs]], [[file-crates-myrn-store-src-graph-rs]], [[file-crates-myrn-store-src-lib-rs]], [[file-crates-myrn-store-src-paths-rs]], [[file-crates-myrn-store-src-state-rs]], [[file-crates-myrn-store-src-sync-rs]], [[file-crates-myrn-store-src-vector-rs]]
