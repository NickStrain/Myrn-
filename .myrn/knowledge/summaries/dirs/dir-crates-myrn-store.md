---
id: dir-crates-myrn-store
type: summary
status: unverified
title: crates/myrn-store/
source: myrn summarize
about: module:crates/myrn-store
anchors:
- module:crates/myrn-store@f212d606fe5b19a0
---
# crates/myrn-store/

Storage layer for Myrn: Markdown notes in notes/ are the source of truth for earned knowledge, while memory.db (SQLite code graph plus FTS search index) is always rebuildable from code and notes. Writes go note first, then index. db.rs opens the database and runs migrations, graph.rs and fts.rs manage nodes, edges and search rows, sync.rs keeps notes and graph in step, episodes.rs writes append-only JSONL logs, state.rs holds per-project state outside the repo, and paths.rs defines where all of it lives.

## Connections

- **In:** [[dir-crates]]
- **Directories:** [[dir-crates-myrn-store-src]]
