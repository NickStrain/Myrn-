---
id: engine-show
type: summary
status: unverified
title: Engine::show
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::show
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::show@a1ef9e84ef6e8c6e
map_version: '2'
---
# Engine::show

Returns one node with all its edges.

## Behaviour

- Ambiguous and not-found results from pick are passed through unchanged

## Terms

node details, edges, inspect node, node lookup

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:1086-1095`
- **Signature:** `pub fn show(&self, query: &str) -> Result<Lookup<(Node, Vec<graph::Edge>)>>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[engine-pick]], [[graph-edges-of]]
- **Called by:** [[show-run]]
- **Tests:** `notes_become_nodes_edges_anchors_and_summaries`
