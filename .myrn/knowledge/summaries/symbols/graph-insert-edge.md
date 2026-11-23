---
id: graph-insert-edge
type: summary
status: unverified
title: graph::insert_edge
source: myrn summarize
about: sym:crates/myrn-store/src/graph.rs#graph::insert_edge
anchors:
- sym:crates/myrn-store/src/graph.rs#graph::insert_edge@06e84d62e80bcd52
map_version: '2'
---
# graph::insert_edge

Inserts an edge with INSERT OR IGNORE, so a conflicting insert is skipped without error.

## Behaviour

- new edge -> stored with the given status
- conflicting edge -> skipped and the existing row's status is kept
- dst that is not a node -> stored anyway; neighbors later omits it

## Terms

add edge, link, create relation, insert relation, dangling edge, edge write

## Connections

- **Location:** `crates/myrn-store/src/graph.rs:109-121`
- **Signature:** `pub fn insert_edge( conn: &Connection, src: &str, rel: &str, dst: &str, status: Option<&str>, ) -> Result<()>`
- **In:** [[file-crates-myrn-store-src-graph-rs]]
- **Called by:** [[episode-persist]], [[git-index-commit]], [[indexer-persist]]
- **Tests:** `neighbours_both_ways`
