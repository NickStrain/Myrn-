---
id: fts-remove-path
type: summary
status: unverified
title: fts::remove_path
source: myrn summarize
about: sym:crates/myrn-store/src/fts.rs#fts::remove_path
anchors:
- sym:crates/myrn-store/src/fts.rs#fts::remove_path@8481e853bc485b83
map_version: '2'
---
# fts::remove_path

Deletes every search row whose rowid belongs to a nodes row with exactly this path. It must run before those nodes rows are deleted, because it reads nodes to find the rowids.

## Behaviour

- path `src/a.rs` -> deletes search rows for nodes where path equals `src/a.rs` exactly
- no nodes with that path -> nothing deleted and Ok is returned
- called after the nodes rows are gone -> no rowids are found, so stale search rows remain

## Terms

delete, unindex, clear file, drop entries, purge, de-index, file cleanup, forget

## Connections

- **Location:** `crates/myrn-store/src/fts.rs:61-67`
- **Signature:** `pub fn remove_path(conn: &Connection, path: &str) -> Result<()>`
- **In:** [[file-crates-myrn-store-src-fts-rs]]
- **Called by:** [[indexer-delete-file]]
