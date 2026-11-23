---
id: viz-neighbor
type: summary
status: unverified
title: viz::Neighbor
source: myrn summarize
about: sym:crates/myrn-engine/src/viz.rs#viz::Neighbor
anchors:
- sym:crates/myrn-engine/src/viz.rs#viz::Neighbor@f4392a3fa454712f
map_version: '2'
---
# viz::Neighbor

One edge endpoint in the side panel: the relation, the other node's id, its label and its type.

## Behaviour

- if the other id is not a node, label is the raw id and type is 'note'
- ntype serialises under the JSON key 'type'

## Terms

adjacent node, edge endpoint, relation target, linked item

## Connections

- **Location:** `crates/myrn-engine/src/viz.rs:90-96`
- **Signature:** `pub struct Neighbor`
- **In:** [[file-crates-myrn-engine-src-viz-rs]]
