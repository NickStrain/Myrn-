---
id: graph-neighbors
type: summary
status: unverified
title: graph::neighbors
source: myrn summarize
about: sym:crates/myrn-store/src/graph.rs#graph::neighbors
anchors:
- sym:crates/myrn-store/src/graph.rs#graph::neighbors@0ae6b77eecfa5d2a
map_version: '2'
---
# graph::neighbors

Returns the nodes one hop away over one relation in the given direction, paired with the edge status.

## Behaviour

- Dir::Out from 'a' over 'calls' -> dst nodes of a's 'calls' edges
- edge whose other end is missing from nodes -> omitted (inner JOIN)
- results ordered by path, then start_line

## Terms

adjacency, callers, callees, traversal, one hop, related nodes, neighbour lookup

## Connections

- **Location:** `crates/myrn-store/src/graph.rs:150-174`
- **Signature:** `pub fn neighbors( conn: &Connection, id: &str, rel: &str, dir: Dir, ) -> Result<Vec<(Node, Option<String>)>>`
- **In:** [[file-crates-myrn-store-src-graph-rs]]
- **Calls:** [[node-from-row]]
- **Called by:** [[engine-commits-touching]], [[engine-symbol-card]], [[engine-known-about]], [[engine-file-card]], [[engine-impact]], [[engine-history]], [[rank-rank]], [[engine-recall]], [[walks-direct-tests]], [[walks-callers]], [[engine-review-items]], [[engine-summarize]], [[engine-map-body]]
- **Tests:** `neighbours_both_ways`
