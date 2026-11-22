---
id: node
type: summary
status: unverified
title: node
source: myrn summarize
about: sym:crates/myrn-engine/tests/notes.rs#node
anchors:
- sym:crates/myrn-engine/tests/notes.rs#node@e5e1b20f2f7db26d
---
# node

Looks up a graph node by id through graph::node on the engine's DB connection, returning None when absent; panics on DB error.

## Connections

- **Location:** `crates/myrn-engine/tests/notes.rs:93-95`
- **Signature:** `fn node(e: &Engine, id: &str) -> Option<graph::Node>`
- **In:** [[file-crates-myrn-engine-tests-notes-rs]]
- **Calls:** [[engine-db]], [[graph-node-graph]]
- **Called by:** [[body-hash]]
- **Tests:** `notes_become_nodes_edges_anchors_and_summaries`, `malformed_notes_and_dangling_links_are_flagged_not_dropped`, `only_changed_notes_are_reindexed_including_hand_edits`, `links_and_summaries_survive_code_changes`, `write_note_writes_first_then_indexes_and_shared_wins`
