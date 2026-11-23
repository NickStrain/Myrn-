---
id: indexer-delete-file
type: summary
status: unverified
title: indexer::delete_file
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/mod.rs#indexer::delete_file
anchors:
- sym:crates/myrn-engine/src/indexer/mod.rs#indexer::delete_file@2a0f0f0571ed9c54
map_version: '2'
---
# indexer::delete_file

Removes a file's search entries, its non-module nodes, edges whose source is one of those nodes, its refs, and its files row.

## Behaviour

- module nodes are kept; gc_modules prunes them later
- edges from other files pointing into this file are not removed here
- refresh calls this before persist, so a re-parsed file starts clean

## Terms

remove, clean up, purge, drop file, forget file, cascade delete

## Connections

- **Location:** `crates/myrn-engine/src/indexer/mod.rs:196-206`
- **Signature:** `fn delete_file(conn: &Connection, path: &str) -> Result<()>`
- **In:** [[file-crates-myrn-engine-src-indexer-mod-rs]]
- **Calls:** [[fts-remove-path]]
- **Called by:** [[indexer-refresh]]
