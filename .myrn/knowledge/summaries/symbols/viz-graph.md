---
id: viz-graph
type: summary
status: unverified
title: viz::Graph
source: myrn summarize
about: sym:crates/myrn-engine/src/viz.rs#viz::Graph
anchors:
- sym:crates/myrn-engine/src/viz.rs#viz::Graph@96d82b38225f0e77
map_version: '2'
---
# viz::Graph

Carries a drawing as nodes, weighted links, and a count of nodes left out to keep it readable.

## Behaviour

- Default -> no nodes, no links, truncated 0
- truncated is above 0 only when the focus neighbourhood hits its node cap of 400
- all other builders set truncated to 0

## Terms

graph payload, drawing data, node cap, truncated count, weighted links

## Connections

- **Location:** `crates/myrn-engine/src/viz.rs:19-24`
- **Signature:** `pub struct Graph`
- **In:** [[file-crates-myrn-engine-src-viz-rs]]
