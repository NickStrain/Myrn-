---
id: engine-viz-code-graph
type: summary
status: unverified
title: Engine::viz_code_graph
source: myrn summarize
about: sym:crates/myrn-engine/src/viz.rs#viz::Engine::viz_code_graph
anchors:
- sym:crates/myrn-engine/src/viz.rs#viz::Engine::viz_code_graph@4fbc0867779647ab
map_version: '2'
---
# Engine::viz_code_graph

Builds the code graph at file level, directory level, or as a focus neighbourhood around one node.

## Behaviour

- level 'focus' with an id -> neighbourhood, depth clamped to 1..3, cap 400
- level 'dirs' -> module nodes with part_of links, plus import links between different folders
- any other level, or 'focus' with no id -> file nodes with import, call and test links; self-imports skipped

## Terms

file graph, directory graph, focus view, dependency links, call graph, test links

## Connections

- **Location:** `crates/myrn-engine/src/viz.rs:168-249`
- **Signature:** `pub fn viz_code_graph(&self, level: &str, focus: Option<&str>, depth: usize) -> Result<Graph>`
- **In:** [[file-crates-myrn-engine-src-viz-rs]]
- **Calls:** [[engine-viz-neighborhood]], [[viz-edges]], [[graph-nodes-where]]
- **Tests:** `code_graph_links_files_by_imports_and_calls`
