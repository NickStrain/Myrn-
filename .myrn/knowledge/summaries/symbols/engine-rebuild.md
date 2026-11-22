---
id: engine-rebuild
type: summary
status: unverified
title: Engine::rebuild
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::rebuild
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::rebuild@31b599c230143816
---
# Engine::rebuild

Deletes the database file with its -wal and -shm files, reopens a fresh database, refreshes and re-persists episodes. Note files are only read.

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:322-335`
- **Signature:** `pub fn rebuild(&mut self) -> Result<IndexReport>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[engine-refresh]], [[engine-reindex-episodes]], [[db-open]], [[db-open-in-memory]]
- **Called by:** [[rebuild-run]]
- **Tests:** `incremental_refresh_tracks_edits_and_deletes`, `rebuild_recreates_the_same_index_from_code_and_notes`, `bugs_are_stored_deduplicated_and_recalled`
