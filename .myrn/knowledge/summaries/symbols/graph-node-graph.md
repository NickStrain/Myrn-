---
id: graph-node-graph
type: summary
status: unverified
title: graph::node
source: myrn summarize
about: sym:crates/myrn-store/src/graph.rs#graph::node
anchors:
- sym:crates/myrn-store/src/graph.rs#graph::node@6ce77ba21fc312b9
---
# graph::node

Fetches one node by id, returning Ok(None) when absent rather than an error.

## Connections

- **Location:** `crates/myrn-store/src/graph.rs:123-131`
- **Signature:** `pub fn node(conn: &Connection, id: &str) -> Result<Option<Node>>`
- **In:** [[file-crates-myrn-store-src-graph-rs]]
- **Called by:** [[engine-commits-touching]], [[engine-history]], [[engine-context-excluding]], [[engine-brief]], [[engine-shell-changes]], [[episode-persist]], [[engine-resolve-ref]], [[engine-records-request]], [[git-index-history]], [[rank-record-weight]], [[seed-find]], [[seed-seeds]], [[engine-review-items]], [[node]], [[node-records]], [[resolver-exists]]
