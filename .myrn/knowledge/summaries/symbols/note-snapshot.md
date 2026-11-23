---
id: note-snapshot
type: summary
status: unverified
title: note_snapshot
source: myrn summarize
about: sym:crates/myrn-engine/tests/notes.rs#note_snapshot
anchors:
- sym:crates/myrn-engine/tests/notes.rs#note_snapshot@9ac1cfd51e4ab448
map_version: '2'
---
# note_snapshot

Builds a sorted list of string lines covering all note-derived index state, so two indexes can be compared directly.

## Behaviour

- earned non-episode nodes -> 'id|type|title|status|doc' lines
- note-evidence edges -> 'src rel dst' lines, anchors -> 'note symbol hash' with '-' for nulls
- summaries, note_issues, and nodes matching the full-text term 'unhashable' are included too

## Terms

index snapshot, note state, comparable dump, index contents, rebuild comparison

## Connections

- **Location:** `crates/myrn-engine/tests/notes.rs:113-133`
- **Signature:** `fn note_snapshot(e: &Engine) -> Vec<String>`
- **In:** [[file-crates-myrn-engine-tests-notes-rs]]
- **Calls:** [[engine-db]]
- **Tests:** `notes_become_nodes_edges_anchors_and_summaries`, `rebuild_recreates_the_same_index_from_code_and_notes`
