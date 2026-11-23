---
id: engine-viz-neighborhood
type: summary
status: unverified
title: Engine::viz_neighborhood
source: myrn summarize
about: sym:crates/myrn-engine/src/viz.rs#viz::Engine::viz_neighborhood
anchors:
- sym:crates/myrn-engine/src/viz.rs#viz::Engine::viz_neighborhood@310a7788d74f75ea
map_version: '2'
---
# Engine::viz_neighborhood

Breadth-first walk from one node over edges for depth steps, capped at max nodes, returning the nodes and the links between them.

## Behaviour

- depth 0 -> only the start node and no links
- a neighbour found after the cap is counted in truncated and not expanded
- a link is kept only when both ends are in the final node set

## Terms

breadth first, neighbourhood, depth limit, node cap, truncation

## Connections

- **Location:** `crates/myrn-engine/src/viz.rs:252-293`
- **Signature:** `fn viz_neighborhood(&self, id: &str, depth: usize, max: usize) -> Result<Graph>`
- **In:** [[file-crates-myrn-engine-src-viz-rs]]
- **Calls:** [[viz-gnode-viz]], [[graph-node-graph]], [[graph-edges-of]]
- **Called by:** [[engine-viz-code-graph]]
