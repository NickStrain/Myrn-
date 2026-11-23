---
id: graph-upsert-node
type: summary
status: unverified
title: graph::upsert_node
source: myrn summarize
about: sym:crates/myrn-store/src/graph.rs#graph::upsert_node
anchors:
- sym:crates/myrn-store/src/graph.rs#graph::upsert_node@0e2fcc4c6b4ca0af
map_version: '2'
---
# graph::upsert_node

Writes a node with INSERT OR REPLACE keyed on id and returns the rowid of that insert.

## Behaviour

- new id -> row inserted, its rowid returned
- existing id -> old row replaced, returns the rowid of the new insert
- None fields are stored as NULL

## Terms

upsert, insert or replace, save node, store node, rowid, search index key, write node

## Connections

- **Location:** `crates/myrn-store/src/graph.rs:82-107`
- **Signature:** `pub fn upsert_node(conn: &Connection, n: &Node) -> Result<i64>`
- **In:** [[file-crates-myrn-store-src-graph-rs]]
- **Called by:** [[episode-persist]], [[git-index-commit]], [[indexer-persist]], [[sync-link]]
- **Tests:** `neighbours_both_ways`
