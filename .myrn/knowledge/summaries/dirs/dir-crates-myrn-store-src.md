---
id: dir-crates-myrn-store-src
type: summary
status: unverified
title: crates/myrn-store/src/
source: myrn summarize
about: module:crates/myrn-store/src
anchors:
- module:crates/myrn-store/src@0a46ba20d3db750c
---
# crates/myrn-store/src/

Storage layer for Myrn, split into the SQLite side (db.rs opens memory.db and runs migrations; graph.rs holds nodes and edges; fts.rs is the FTS index keyed by nodes.rowid; vector.rs is an empty placeholder) and the file side (notes/ holds the Markdown knowledge notes, which are the source of truth; episodes.rs keeps per-session JSONL logs; state.rs keeps private state.json; paths.rs resolves Layout). sync.rs keeps the notes table and note-derived graph rows in step with note files on disk, and lib.rs re-exports the public types.

## Connections

- **In:** [[dir-crates-myrn-store]]
- **Directories:** [[dir-crates-myrn-store-src-notes]]
- **Files:** [[file-crates-myrn-store-src-db-rs]], [[file-crates-myrn-store-src-episodes-rs]], [[file-crates-myrn-store-src-fts-rs]], [[file-crates-myrn-store-src-graph-rs]], [[file-crates-myrn-store-src-lib-rs]], [[file-crates-myrn-store-src-paths-rs]], [[file-crates-myrn-store-src-state-rs]], [[file-crates-myrn-store-src-sync-rs]], [[file-crates-myrn-store-src-vector-rs]]
