---
id: viz-map-sort
type: summary
status: unverified
title: viz_map::sort
source: myrn summarize
about: sym:crates/myrn-engine/src/viz.rs#viz::Engine::viz_map::sort
anchors:
- sym:crates/myrn-engine/src/viz.rs#viz::Engine::viz_map::sort@01051648ffee2e1b
map_version: '2'
---
# viz_map::sort

Recursively sorts a map entry's children: modules first, then files, then everything else, each by line then name.

## Behaviour

- rank: module 0, file 1, anything else 2
- entries with no line sort before entries with one
- applies to every level below the entry

## Terms

child order, tree ordering, rank, line number, name tiebreak

## Connections

- **Location:** `crates/myrn-engine/src/viz.rs:526-541`
- **Signature:** `fn sort(e: &mut MapEntry)`
- **In:** [[file-crates-myrn-engine-src-viz-rs]]
- **Calls:** [[rank-rank]], [[viz-map-sort]]
- **Called by:** [[engine-viz-map]], [[viz-map-sort]]
