---
id: notes-graph-map-records-and-details-find
type: summary
status: unverified
title: notes_graph_map_records_and_details::find
source: myrn summarize
about: sym:crates/myrn-engine/tests/viz.rs#notes_graph_map_records_and_details::find
anchors:
- sym:crates/myrn-engine/tests/viz.rs#notes_graph_map_records_and_details::find@3bc646cbe8fa320b
map_version: '2'
---
# notes_graph_map_records_and_details::find

Depth-first search of a MapEntry tree for the first entry whose name equals the given name exactly. Returns None if no entry matches.

## Behaviour

- find(root, 'chunked') -> the first entry named 'chunked', checking each node before its children
- find(root, 'pkg.more.chunked') -> None unless an entry has exactly that name; no prefix or partial matching
- name equal to the root's name -> returns the root itself

## Terms

tree search, map lookup, depth first, recursive search, child search, entry lookup, project map node

## Connections

- **Location:** `crates/myrn-engine/tests/viz.rs:109-117`
- **Signature:** `fn find<'a>( e: &'a myrn_engine::viz::MapEntry, name: &str, ) -> Option<&'a myrn_engine::viz::MapEntry>`
- **In:** [[file-crates-myrn-engine-tests-viz-rs]]
- **Calls:** [[notes-graph-map-records-and-details-find]]
- **Called by:** [[notes-graph-map-records-and-details-find]]
- **Tests:** `notes_graph_map_records_and_details`
