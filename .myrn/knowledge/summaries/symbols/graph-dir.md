---
id: graph-dir
type: summary
status: unverified
title: graph::Dir
source: myrn summarize
about: sym:crates/myrn-store/src/graph.rs#graph::Dir
anchors:
- sym:crates/myrn-store/src/graph.rs#graph::Dir@a3bece0928c8bd9f
map_version: '2'
---
# graph::Dir

Selects edge direction for traversal: Out follows src to dst, In follows dst to src.

## Behaviour

- Out -> edges where the node is src
- In -> edges where the node is dst
- used only by neighbors

## Terms

direction, outgoing, incoming, edge direction, traversal direction

## Connections

- **Location:** `crates/myrn-store/src/graph.rs:144-147`
- **Signature:** `pub enum Dir`
- **In:** [[file-crates-myrn-store-src-graph-rs]]
