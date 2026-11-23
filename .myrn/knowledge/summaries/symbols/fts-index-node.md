---
id: fts-index-node
type: summary
status: unverified
title: fts::index_node
source: myrn summarize
about: sym:crates/myrn-store/src/fts.rs#fts::index_node
anchors:
- sym:crates/myrn-store/src/fts.rs#fts::index_node@d0212f6a21bf1b45
map_version: '2'
---
# fts::index_node

Inserts one search row for a node: terms from qname then name, path words, and the raw doc text. It is a plain INSERT with no upsert, and it returns the database error if the insert fails.

## Behaviour

- qname `TokenStore::lookup` with name `lookup` -> terms `token store lookup`, since dedup removes the adjacent repeated word
- path `src/auth/session.rs` -> path column `src auth session rs`; doc is stored unsplit
- dedup only removes adjacent duplicates, so non-adjacent repeats stay in the terms column

## Terms

insert, add entry, search row, populate, index entry, rowid, FTS insert, store node

## Connections

- **Location:** `crates/myrn-store/src/fts.rs:36-58`
- **Signature:** `pub fn index_node( conn: &Connection, rowid: i64, name: &str, qname: &str, path: &str, doc: &str, ) -> Result<()>`
- **In:** [[file-crates-myrn-store-src-fts-rs]]
- **Calls:** [[fts-split-identifier]]
- **Called by:** [[git-index-commit]], [[indexer-persist]], [[sync-link]]
