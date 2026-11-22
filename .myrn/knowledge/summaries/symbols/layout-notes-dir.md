---
id: layout-notes-dir
type: summary
status: unverified
title: Layout::notes_dir
source: myrn summarize
about: sym:crates/myrn-store/src/paths.rs#paths::Layout::notes_dir
anchors:
- sym:crates/myrn-store/src/paths.rs#paths::Layout::notes_dir@8daae0ebd3462e2c
---
# Layout::notes_dir

Returns data_dir/notes, the private per-user notes tree.

## Connections

- **Location:** `crates/myrn-store/src/paths.rs:69-71`
- **Signature:** `pub fn notes_dir(&self) -> PathBuf`
- **In:** [[file-crates-myrn-store-src-paths-rs]]
- **Called by:** [[engine-note-roots]], [[engine-write-note]], [[engine-put-note]], [[engine-note-file]], [[indexed-repo]], [[layout-create-dirs]]
- **Tests:** `only_changed_notes_are_reindexed_including_hand_edits`, `bugs_are_stored_deduplicated_and_recalled`, `contradictions_are_flagged_and_superseded_records_drop_out`, `one_call_records_many_items_and_trims_test_runs`
