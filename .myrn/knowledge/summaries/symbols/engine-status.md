---
id: engine-status
type: summary
status: unverified
title: Engine::status
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::status
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::status@064557c8ac7f99bc
---
# Engine::status

Collects the counts for Status: files by language and with parse errors, symbols, tests, modules, commits, edges by relation, inferred calls, earned records per state (excluding episodes), the note summary, connected agents and last refresh time.

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:1031-1083`
- **Signature:** `pub fn status(&self) -> Result<Status>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[engine-state]], [[db-get-meta]], [[graph-count]], [[sync-summary]]
- **Called by:** [[engine-brief]], [[git-index-commit]], [[init-run]], [[status-run]]
- **Tests:** `incremental_refresh_tracks_edits_and_deletes`, `notes_become_nodes_edges_anchors_and_summaries`, `malformed_notes_and_dangling_links_are_flagged_not_dropped`, `only_changed_notes_are_reindexed_including_hand_edits`, `write_note_writes_first_then_indexes_and_shared_wins`, `rejections_say_what_to_fix`
