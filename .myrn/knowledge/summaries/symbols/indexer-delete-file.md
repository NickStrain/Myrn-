---
id: indexer-delete-file
type: summary
status: unverified
title: indexer::delete_file
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/mod.rs#indexer::delete_file
anchors:
- sym:crates/myrn-engine/src/indexer/mod.rs#indexer::delete_file@2a0f0f0571ed9c54
---
# indexer::delete_file

Removes all derived data for one path: its FTS entry, its non-module nodes, edges sourced from those nodes, refs, and its files row. Module nodes are kept. Callers must run this before persist when re-indexing a file.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/mod.rs:196-206`
- **Signature:** `fn delete_file(conn: &Connection, path: &str) -> Result<()>`
- **In:** [[file-crates-myrn-engine-src-indexer-mod-rs]]
- **Calls:** [[fts-remove-path]]
- **Called by:** [[indexer-refresh]]
