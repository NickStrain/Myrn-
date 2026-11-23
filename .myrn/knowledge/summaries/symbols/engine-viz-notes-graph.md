---
id: engine-viz-notes-graph
type: summary
status: unverified
title: Engine::viz_notes_graph
source: myrn summarize
about: sym:crates/myrn-engine/src/viz.rs#viz::Engine::viz_notes_graph
anchors:
- sym:crates/myrn-engine/src/viz.rs#viz::Engine::viz_notes_graph@ce5eded4e2f0e1c0
map_version: '2'
---
# Engine::viz_notes_graph

Builds the notes graph: one node per note, linked by header links and wikilinks, with code nodes added when code is true.

## Behaviour

- duplicate note ids keep one row; shared copies are read first
- link targets match note ids case-insensitively after trimming; unmatched targets are dropped
- with code true, 'about' links from summaries and note-to-code edges add code nodes grouped as 'code'

## Terms

notes graph, wikilinks, header links, case-insensitive match, code nodes, note edges

## Connections

- **Location:** `crates/myrn-engine/src/viz.rs:298-409`
- **Signature:** `pub fn viz_notes_graph(&self, code: bool) -> Result<Graph>`
- **In:** [[file-crates-myrn-engine-src-viz-rs]]
- **Calls:** [[viz-gnode-viz]], [[viz-edges3]], [[graph-node-graph]], [[note-display-title]], [[wikilink-extract]]
- **Tests:** `notes_graph_map_records_and_details`
