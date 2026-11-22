---
id: graph-upsert-node
type: summary
status: unverified
title: graph::upsert_node
source: myrn summarize
about: sym:crates/myrn-store/src/graph.rs#graph::upsert_node
anchors:
- sym:crates/myrn-store/src/graph.rs#graph::upsert_node@0e2fcc4c6b4ca0af
---
# graph::upsert_node

INSERT OR REPLACE into nodes and returns last_insert_rowid. Replacing an existing row deletes and reinserts it, so the returned rowid can change and callers must not cache it as stable.

## Connections

- **Location:** `crates/myrn-store/src/graph.rs:82-107`
- **Signature:** `pub fn upsert_node(conn: &Connection, n: &Node) -> Result<i64>`
- **In:** [[file-crates-myrn-store-src-graph-rs]]
- **Called by:** [[episode-persist]], [[git-index-commit]], [[indexer-persist]], [[sync-link]]
- **Tests:** `neighbours_both_ways`
