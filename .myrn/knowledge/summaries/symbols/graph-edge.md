---
id: graph-edge
type: summary
status: unverified
title: graph::Edge
source: myrn summarize
about: sym:crates/myrn-store/src/graph.rs#graph::Edge
anchors:
- sym:crates/myrn-store/src/graph.rs#graph::Edge@bc1356351a203467
map_version: '2'
---
# graph::Edge

Holds one directed edge row: src, rel, dst and an optional status.

## Behaviour

- status None when the edge has no status value
- rel is free text, e.g. 'calls', and is not validated
- dst is not checked to exist as a node

## Terms

relation, link, dependency, arc, directed edge

## Connections

- **Location:** `crates/myrn-store/src/graph.rs:29-34`
- **Signature:** `pub struct Edge`
- **In:** [[file-crates-myrn-store-src-graph-rs]]
