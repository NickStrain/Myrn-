---
id: engine-refresh
type: summary
status: unverified
title: Engine::refresh
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::refresh
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::refresh@2185770cbad7c568
map_version: '2'
---
# Engine::refresh

Re-indexes changed files, syncs notes, re-links note edges when code changed, updates embeddings when enabled, and returns the index report with elapsed milliseconds.

## Behaviour

- Note links are re-linked when code was parsed or removed, or when the notes sync reports relinked
- A failed embedding update prints a message and does not fail the refresh
- Sets last_refresh to now

## Terms

reindex, sync, update index, incremental update, re-link notes, embeddings

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:256-277`
- **Signature:** `pub fn refresh(&mut self) -> Result<IndexReport>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[engine-note-roots]], [[engine-embeddings-on]], [[engine-update-vectors]], [[indexer-refresh]], [[layout-ignore-path]], [[sync-sync]]
- **Called by:** [[engine-init]], [[engine-refresh-if-older]], [[engine-rebuild]], [[engine-shell-changes]], [[engine-touched-symbols]], [[engine-doctor]], [[engine-summarize]], [[engine-gc]], [[indexed-repo]], [[set-validity]], [[hook-handle]], [[ctx-engine]]
- **Tests:** `incremental_refresh_tracks_edits_and_deletes`, `log_lists_records_and_trace_shows_where_they_came_from`, `diff_compares_shared_knowledge_between_points`, `gc_merges_copies_drops_stale_rows_and_compacts_old_sessions`, `malformed_notes_and_dangling_links_are_flagged_not_dropped`, `only_changed_notes_are_reindexed_including_hand_edits`
