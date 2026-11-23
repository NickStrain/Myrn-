---
id: graph-node-graph
type: summary
status: unverified
title: graph::node
source: myrn summarize
about: sym:crates/myrn-store/src/graph.rs#graph::node
anchors:
- sym:crates/myrn-store/src/graph.rs#graph::node@6ce77ba21fc312b9
map_version: '2'
---
# graph::node

Fetches one node by id, returning None when no row matches.

## Behaviour

- id present -> Some(Node)
- id absent -> None, not an error
- SQL or mapping failure -> Err

## Terms

lookup, get by id, fetch node, find node, single node

## Connections

- **Location:** `crates/myrn-store/src/graph.rs:123-131`
- **Signature:** `pub fn node(conn: &Connection, id: &str) -> Result<Option<Node>>`
- **In:** [[file-crates-myrn-store-src-graph-rs]]
- **Called by:** [[engine-commits-touching]], [[engine-history]], [[engine-context-excluding]], [[engine-brief]], [[engine-shell-changes]], [[episode-persist]], [[engine-resolve-ref]], [[engine-records-request]], [[git-index-history]], [[engine-records]], [[engine-record-row]], [[engine-trace]], [[engine-forget-plan]], [[rank-record-weight]], [[seed-find]], [[seed-seeds]], [[engine-review-items]], [[engine-publish-plan]], [[engine-suspect-notice]], [[engine-suspects]], [[engine-changed]], [[engine-confirm]], [[engine-viz-neighborhood]], [[engine-viz-notes-graph]], [[engine-viz-details]]
