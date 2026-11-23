---
id: engine-rebuild
type: summary
status: unverified
title: Engine::rebuild
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::rebuild
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::rebuild@c3fed040e24100ed
map_version: '2'
---
# Engine::rebuild

Deletes the database file with its wal and shm files, reopens it, restores local verifications, refreshes, and re-indexes episodes. Note files are only read.

## Behaviour

- Local verifications (note_id, body_hash, reason, t) are copied before deletion and written back after reopening
- If that table cannot be read, nothing is restored and the rebuild still continues

## Terms

reset index, recreate database, wipe index, verifications, rebuild from code

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:330-360`
- **Signature:** `pub fn rebuild(&mut self) -> Result<IndexReport>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[engine-refresh]], [[engine-reindex-episodes]], [[db-open]], [[db-open-in-memory]]
- **Called by:** [[rebuild-run]]
- **Tests:** `incremental_refresh_tracks_edits_and_deletes`, `rebuild_recreates_the_same_index_from_code_and_notes`, `bugs_are_stored_deduplicated_and_recalled`, `imported_records_start_unverified_whatever_they_say`, `records_become_verified_when_a_cited_test_passes_or_the_fix_lands`
