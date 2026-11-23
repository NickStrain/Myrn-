---
id: viz-mapdata
type: summary
status: unverified
title: viz::MapData
source: myrn summarize
about: sym:crates/myrn-engine/src/viz.rs#viz::MapData
anchors:
- sym:crates/myrn-engine/src/viz.rs#viz::MapData@c8aa28f0cffe0c1a
map_version: '2'
---
# viz::MapData

Project map result: the root tree plus counts of fresh, stale and missing summaries over files and symbols.

## Behaviour

- modules are left out of the three counts
- root is None when there are no module or file nodes
- when no root directory node exists, a synthetic '.' root holds the remaining entries

## Terms

map summary, freshness counts, root tree, stale count, missing summaries

## Connections

- **Location:** `crates/myrn-engine/src/viz.rs:70-76`
- **Signature:** `pub struct MapData`
- **In:** [[file-crates-myrn-engine-src-viz-rs]]
