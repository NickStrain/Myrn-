---
id: node-from-row
type: summary
status: unverified
title: Node::from_row
source: myrn summarize
about: sym:crates/myrn-store/src/graph.rs#graph::Node::from_row
anchors:
- sym:crates/myrn-store/src/graph.rs#graph::Node::from_row@5bf5569aa6f4ab17
map_version: '2'
---
# Node::from_row

Builds a Node from a rusqlite Row by reading columns 0 to 15 in NODE_COLUMNS order.

## Behaviour

- row with columns in NODE_COLUMNS order -> Node
- column in the wrong position or wrong type -> rusqlite error, returned as Err
- does not read any column past index 15

## Terms

row mapping, deserialise, hydrate, column mapping, from sql row

## Connections

- **Location:** `crates/myrn-store/src/graph.rs:40-59`
- **Signature:** `pub fn from_row(r: &Row<'_>) -> rusqlite::Result<Node>`
- **In:** [[file-crates-myrn-store-src-graph-rs]]
- **Called by:** [[graph-neighbors]]
