---
id: engine-viz-map
type: summary
status: unverified
title: Engine::viz_map
source: myrn summarize
about: sym:crates/myrn-engine/src/viz.rs#viz::Engine::viz_map
anchors:
- sym:crates/myrn-engine/src/viz.rs#viz::Engine::viz_map@c05c99c1d9ee3216
map_version: '2'
---
# Engine::viz_map

Builds the project map tree of modules, files and symbols, nested by part_of, with each summary marked fresh, stale or missing.

## Behaviour

- a summary is stale when its anchor hash is set and the body hash does not start with it; no anchor means fresh
- a node with a blank or absent summary is missing
- symbols are kept only for function, method, class, struct, enum, trait, type and macro kinds

## Terms

project tree, summary freshness, stale summary, anchor hash, directory tree, map nesting

## Connections

- **Location:** `crates/myrn-engine/src/viz.rs:412-570`
- **Signature:** `pub fn viz_map(&self) -> Result<MapData>`
- **In:** [[file-crates-myrn-engine-src-viz-rs]]
- **Calls:** [[viz-map-sort]], [[viz-edges]], [[id-file-id]], [[id-module-id]], [[graph-nodes-where]]
- **Tests:** `notes_graph_map_records_and_details`
