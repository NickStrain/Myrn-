---
id: has-edge
type: summary
status: unverified
title: has_edge
source: myrn summarize
about: sym:crates/myrn-engine/tests/notes.rs#has_edge
anchors:
- sym:crates/myrn-engine/tests/notes.rs#has_edge@7238e2732f048541
map_version: '2'
---
# has_edge

Returns true when an edge row exists with the exact source id, relation name and destination id.

## Behaviour

- ('bug-7f3a91', 'related', 'fact-c0ffee') with that edge present -> true
- matching is exact and case-sensitive, with no wildcards
- absent edge or any database error -> false, so errors are hidden

## Terms

edge exists, relation check, edge lookup, link present, typed edge

## Connections

- **Location:** `crates/myrn-engine/tests/notes.rs:97-106`
- **Signature:** `fn has_edge(e: &Engine, src: &str, rel: &str, dst: &str) -> bool`
- **In:** [[file-crates-myrn-engine-tests-notes-rs]]
- **Calls:** [[engine-db]]
- **Tests:** `notes_become_nodes_edges_anchors_and_summaries`, `malformed_notes_and_dangling_links_are_flagged_not_dropped`, `only_changed_notes_are_reindexed_including_hand_edits`, `links_and_summaries_survive_code_changes`, `write_note_writes_first_then_indexes_and_shared_wins`
