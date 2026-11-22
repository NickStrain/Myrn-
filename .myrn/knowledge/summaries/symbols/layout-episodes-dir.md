---
id: layout-episodes-dir
type: summary
status: unverified
title: Layout::episodes_dir
source: myrn summarize
about: sym:crates/myrn-store/src/paths.rs#paths::Layout::episodes_dir
anchors:
- sym:crates/myrn-store/src/paths.rs#paths::Layout::episodes_dir@25aaae9e320468eb
---
# Layout::episodes_dir

Returns data_dir/episodes.

## Connections

- **Location:** `crates/myrn-store/src/paths.rs:72-74`
- **Signature:** `pub fn episodes_dir(&self) -> PathBuf`
- **In:** [[file-crates-myrn-store-src-paths-rs]]
- **Called by:** [[engine-capture]], [[engine-reindex-episodes]], [[engine-episodes]], [[engine-episode-log]], [[engine-recorded-test-run]], [[engine-current-session]], [[engine-records-request]], [[session]], [[layout-create-dirs]]
- **Tests:** `session_end_asks_once_per_batch_of_edits`, `shell_edits_are_captured_and_lead_to_a_record_request`
