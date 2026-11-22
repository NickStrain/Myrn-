---
id: dir-crates-myrn-store
type: summary
status: unverified
title: crates/myrn-store/
source: myrn summarize
about: module:crates/myrn-store
anchors:
- module:crates/myrn-store@10939e0b2e0f5d07
---
# crates/myrn-store/

Storage layer for Myrn, combining a SQLite database (memory.db, opened and migrated by db.rs) holding graph nodes/edges (graph.rs), an FTS index keyed by nodes.rowid (fts.rs), and an empty vector.rs placeholder, with a file side where Markdown notes in notes/ are the source of truth, per-session JSONL episode logs (episodes.rs), and a private state.json (state.rs), with paths.rs resolving Layout. sync.rs keeps the notes table and note-derived graph rows consistent with note files on disk, and lib.rs re-exports the public types.

## Connections

- **In:** [[dir-crates]]
- **Directories:** [[dir-crates-myrn-store-src]]
