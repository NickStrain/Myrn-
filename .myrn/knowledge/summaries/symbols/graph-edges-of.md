---
id: graph-edges-of
type: summary
status: unverified
title: graph::edges_of
source: myrn summarize
about: sym:crates/myrn-store/src/graph.rs#graph::edges_of
anchors:
- sym:crates/myrn-store/src/graph.rs#graph::edges_of@ed825bda2ae05f77
map_version: '2'
---
# graph::edges_of

Returns every edge where the node is src or dst, across all relations, with no status filter.

## Behaviour

- id as src or dst -> all touching edges
- self-loop (src = dst = id) -> returned twice
- unknown id -> empty Vec; dangling edges are included

## Terms

incident edges, all links, edges touching node, adjacency list, node edges, edge listing

## Connections

- **Location:** `crates/myrn-store/src/graph.rs:177-192`
- **Signature:** `pub fn edges_of(conn: &Connection, id: &str) -> Result<Vec<Edge>>`
- **In:** [[file-crates-myrn-store-src-graph-rs]]
- **Called by:** [[engine-show]], [[engine-viz-neighborhood]], [[engine-viz-details]]
