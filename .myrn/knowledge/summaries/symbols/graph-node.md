---
id: graph-node
type: summary
status: unverified
title: graph::Node
source: myrn summarize
about: sym:crates/myrn-store/src/graph.rs#graph::Node
anchors:
- sym:crates/myrn-store/src/graph.rs#graph::Node@5b2e1a687a594dc1
map_version: '2'
---
# graph::Node

Holds one graph node's columns, with ntype serialised as the key 'type'. Option fields are None when the column is NULL.

## Behaviour

- Default -> empty id, ntype and origin, and None for every Option field
- serde output uses key 'type' for ntype
- extra, status and body_hash are opaque strings, not parsed here

## Terms

node record, vertex, entity, row struct, symbol record, graph entry

## Connections

- **Location:** `crates/myrn-store/src/graph.rs:8-26`
- **Signature:** `pub struct Node`
- **In:** [[file-crates-myrn-store-src-graph-rs]]
