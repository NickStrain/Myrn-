---
id: graph-edges-of
type: summary
status: unverified
title: graph::edges_of
source: myrn summarize
about: sym:crates/myrn-store/src/graph.rs#graph::edges_of
anchors:
- sym:crates/myrn-store/src/graph.rs#graph::edges_of@ed825bda2ae05f77
---
# graph::edges_of

Returns every edge where the id is src or dst, with any relation, via UNION ALL. A self-loop appears twice, once per branch.

## Connections

- **Location:** `crates/myrn-store/src/graph.rs:177-192`
- **Signature:** `pub fn edges_of(conn: &Connection, id: &str) -> Result<Vec<Edge>>`
- **In:** [[file-crates-myrn-store-src-graph-rs]]
- **Called by:** [[engine-show]]
