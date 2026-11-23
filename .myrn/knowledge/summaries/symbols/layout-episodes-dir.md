---
id: layout-episodes-dir
type: summary
status: unverified
title: Layout::episodes_dir
source: myrn summarize
about: sym:crates/myrn-store/src/paths.rs#paths::Layout::episodes_dir
anchors:
- sym:crates/myrn-store/src/paths.rs#paths::Layout::episodes_dir@25aaae9e320468eb
map_version: '2'
---
# Layout::episodes_dir

Returns the episodes directory in the per-user data dir.

## Behaviour

- -> <data_dir>/episodes
- created by create_dirs

## Terms

episodes, episode store, episode folder

## Connections

- **Location:** `crates/myrn-store/src/paths.rs:72-74`
- **Signature:** `pub fn episodes_dir(&self) -> PathBuf`
- **In:** [[file-crates-myrn-store-src-paths-rs]]
- **Called by:** [[engine-capture]], [[engine-reindex-episodes]], [[engine-episodes]], [[engine-episode-log]], [[engine-recorded-test-run]], [[engine-current-session]], [[engine-records-request]], [[engine-session-agent]], [[engine-gc]], [[engine-passing-test-runs]], [[session-history]], [[session]], [[layout-create-dirs]]
- **Tests:** `gc_merges_copies_drops_stale_rows_and_compacts_old_sessions`, `sessions_attach_their_outcome`, `session_end_asks_once_per_batch_of_edits`, `shell_edits_are_captured_and_lead_to_a_record_request`, `records_become_verified_when_a_cited_test_passes_or_the_fix_lands`
