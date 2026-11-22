---
id: fts-index-node
type: summary
status: unverified
title: fts::index_node
source: myrn summarize
about: sym:crates/myrn-store/src/fts.rs#fts::index_node
anchors:
- sym:crates/myrn-store/src/fts.rs#fts::index_node@d0212f6a21bf1b45
---
# fts::index_node

Inserts one row into the search table with rowid, raw name, space-joined terms from split(qname) followed by split(name) with adjacent duplicates removed, space-joined split(path), and raw doc. Does not check for an existing row, so call fts::remove_path first when re-indexing.

## Connections

- **Location:** `crates/myrn-store/src/fts.rs:35-57`
- **Signature:** `pub fn index_node( conn: &Connection, rowid: i64, name: &str, qname: &str, path: &str, doc: &str, ) -> Result<()>`
- **In:** [[file-crates-myrn-store-src-fts-rs]]
- **Calls:** [[fts-split-identifier]]
- **Called by:** [[git-index-commit]], [[indexer-persist]], [[sync-link]]
