---
id: graph-insert-edge
type: summary
status: unverified
title: graph::insert_edge
source: myrn summarize
about: sym:crates/myrn-store/src/graph.rs#graph::insert_edge
anchors:
- sym:crates/myrn-store/src/graph.rs#graph::insert_edge@06e84d62e80bcd52
---
# graph::insert_edge

INSERT OR IGNORE into edges, so a duplicate (src, rel, dst) is a silent no-op and the status of the first insert is kept. Does not check that the endpoints exist.

## Connections

- **Location:** `crates/myrn-store/src/graph.rs:109-121`
- **Signature:** `pub fn insert_edge( conn: &Connection, src: &str, rel: &str, dst: &str, status: Option<&str>, ) -> Result<()>`
- **In:** [[file-crates-myrn-store-src-graph-rs]]
- **Called by:** [[episode-persist]], [[git-index-commit]], [[indexer-persist]]
- **Tests:** `neighbours_both_ways`
