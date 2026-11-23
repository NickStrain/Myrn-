---
id: engine-viz-details
type: summary
status: unverified
title: Engine::viz_details
source: myrn summarize
about: sym:crates/myrn-engine/src/viz.rs#viz::Engine::viz_details
anchors:
- sym:crates/myrn-engine/src/viz.rs#viz::Engine::viz_details@aab03f035a73db0f
map_version: '2'
---
# Engine::viz_details

Gathers a node, its note body and note path, and up to 400 of its edges, split into outgoing and incoming with labels.

## Behaviour

- node is None when the id is not in the store
- body is set only when the note data parses, and is passed through safe_text
- neighbour labels fall back to the raw id with type 'note' when the other end is not a node

## Terms

side panel, note body, incoming edges, outgoing edges, neighbour labels, node details

## Connections

- **Location:** `crates/myrn-engine/src/viz.rs:573-623`
- **Signature:** `pub fn viz_details(&self, id: &str) -> Result<Details>`
- **In:** [[file-crates-myrn-engine-src-viz-rs]]
- **Calls:** [[render-safe-text]], [[viz-gnode-viz]], [[graph-node-graph]], [[graph-edges-of]]
- **Tests:** `notes_graph_map_records_and_details`
