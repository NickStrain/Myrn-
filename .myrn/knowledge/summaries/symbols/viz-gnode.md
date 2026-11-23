---
id: viz-gnode
type: summary
status: unverified
title: viz::GNode
source: myrn summarize
about: sym:crates/myrn-engine/src/viz.rs#viz::GNode
anchors:
- sym:crates/myrn-engine/src/viz.rs#viz::GNode@3261b8a500264d4d
map_version: '2'
---
# viz::GNode

One node as the browser receives it: id, display label, type, optional kind, path, status and visibility, plus a colour group.

## Behaviour

- ntype serialises under the JSON key 'type'
- visibility is set only when the extra field holds JSON with a visibility string
- group is computed from path by top_group

## Terms

vertex, node record, colour group, display label, note visibility

## Connections

- **Location:** `crates/myrn-engine/src/viz.rs:27-41`
- **Signature:** `pub struct GNode`
- **In:** [[file-crates-myrn-engine-src-viz-rs]]
