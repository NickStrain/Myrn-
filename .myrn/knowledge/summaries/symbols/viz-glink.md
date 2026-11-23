---
id: viz-glink
type: summary
status: unverified
title: viz::GLink
source: myrn summarize
about: sym:crates/myrn-engine/src/viz.rs#viz::GLink
anchors:
- sym:crates/myrn-engine/src/viz.rs#viz::GLink@21eb8d727c2afff7
map_version: '2'
---
# viz::GLink

One link between two node ids, with a relation name and a count of the underlying edges it stands for.

## Behaviour

- fields serialise as s, t, rel, w
- w is the number of merged edges in code graphs; neighbourhood and notes links use 1
- s and t are node ids, not labels

## Terms

edge, relation, source and target, merged edges, link weight

## Connections

- **Location:** `crates/myrn-engine/src/viz.rs:44-50`
- **Signature:** `pub struct GLink`
- **In:** [[file-crates-myrn-engine-src-viz-rs]]
