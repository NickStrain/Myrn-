---
id: note-snapshot
type: summary
status: unverified
title: note_snapshot
source: myrn summarize
about: sym:crates/myrn-engine/tests/notes.rs#note_snapshot
anchors:
- sym:crates/myrn-engine/tests/notes.rs#note_snapshot@9ac1cfd51e4ab448
---
# note_snapshot

Collects sorted text rows describing what notes contribute: earned non-episode nodes with type, title, status and doc, note-sourced edges, anchors, symbol summaries, note_issues, and a full-text search hit for 'unhashable'. Used to compare the index before and after a rebuild.

## Connections

- **Location:** `crates/myrn-engine/tests/notes.rs:113-133`
- **Signature:** `fn note_snapshot(e: &Engine) -> Vec<String>`
- **In:** [[file-crates-myrn-engine-tests-notes-rs]]
- **Calls:** [[engine-db]]
- **Tests:** `notes_become_nodes_edges_anchors_and_summaries`, `rebuild_recreates_the_same_index_from_code_and_notes`
