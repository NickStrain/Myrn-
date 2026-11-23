---
id: engine-status
type: summary
status: unverified
title: Engine::status
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::status
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::status@b7ab51ea1ee6c485
map_version: '2'
---
# Engine::status

Gathers counts and states from the store: files by language, parse errors, symbols, tests, modules, commits, edges, inferred calls, record states, notes, conflicts, agents and last refresh.

## Behaviour

- Record states list every status, with 0 for statuses that have no records
- Episode nodes are excluded from the record counts
- last_refresh is read from meta and is None when unset or unparseable

## Terms

index stats, counts, health summary, store status, store counts

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:1097-1150`
- **Signature:** `pub fn status(&self) -> Result<Status>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[engine-state]], [[db-get-meta]], [[graph-count]], [[sync-conflicts]], [[sync-summary]]
- **Called by:** [[engine-brief]], [[git-index-commit]], [[init-run]], [[status-run]], [[ui-serve]]
- **Tests:** `incremental_refresh_tracks_edits_and_deletes`, `notes_become_nodes_edges_anchors_and_summaries`, `malformed_notes_and_dangling_links_are_flagged_not_dropped`, `only_changed_notes_are_reindexed_including_hand_edits`, `write_note_writes_first_then_indexes_and_shared_wins`, `rejections_say_what_to_fix`
