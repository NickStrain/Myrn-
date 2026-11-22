---
id: node-from-row
type: summary
status: unverified
title: Node::from_row
source: myrn summarize
about: sym:crates/myrn-store/src/graph.rs#graph::Node::from_row
anchors:
- sym:crates/myrn-store/src/graph.rs#graph::Node::from_row@5bf5569aa6f4ab17
---
# Node::from_row

Builds a Node from a rusqlite Row by positional index 0-15, so the SELECT column list must match NODE_COLUMNS exactly or fields are silently mis-mapped.

## Connections

- **Location:** `crates/myrn-store/src/graph.rs:40-59`
- **Signature:** `pub fn from_row(r: &Row<'_>) -> rusqlite::Result<Node>`
- **In:** [[file-crates-myrn-store-src-graph-rs]]
- **Called by:** [[graph-neighbors]]
