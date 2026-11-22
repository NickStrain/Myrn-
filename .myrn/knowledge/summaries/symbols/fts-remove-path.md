---
id: fts-remove-path
type: summary
status: unverified
title: fts::remove_path
source: myrn summarize
about: sym:crates/myrn-store/src/fts.rs#fts::remove_path
anchors:
- sym:crates/myrn-store/src/fts.rs#fts::remove_path@8481e853bc485b83
---
# fts::remove_path

Deletes search rows whose rowid matches any nodes row with the given path. It reads the nodes table, so it must run before those node rows are deleted.

## Connections

- **Location:** `crates/myrn-store/src/fts.rs:60-66`
- **Signature:** `pub fn remove_path(conn: &Connection, path: &str) -> Result<()>`
- **In:** [[file-crates-myrn-store-src-fts-rs]]
- **Called by:** [[indexer-delete-file]]
