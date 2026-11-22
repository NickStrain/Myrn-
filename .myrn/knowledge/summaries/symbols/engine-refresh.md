---
id: engine-refresh
type: summary
status: unverified
title: Engine::refresh
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::refresh
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::refresh@abe6592b266d7a30
---
# Engine::refresh

Re-indexes the working tree and then syncs the notes, re-linking them when files were parsed or removed. It writes to the database and stamps the last refresh time.

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:254-269`
- **Signature:** `pub fn refresh(&mut self) -> Result<IndexReport>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[engine-note-roots]], [[indexer-refresh]], [[layout-ignore-path]], [[sync-sync]]
- **Called by:** [[engine-init]], [[engine-refresh-if-older]], [[engine-rebuild]], [[engine-shell-changes]], [[engine-touched-symbols]], [[engine-doctor]], [[engine-summarize]], [[indexed-repo]], [[hook-handle]], [[ctx-engine]]
- **Tests:** `incremental_refresh_tracks_edits_and_deletes`, `malformed_notes_and_dangling_links_are_flagged_not_dropped`, `only_changed_notes_are_reindexed_including_hand_edits`, `links_and_summaries_survive_code_changes`, `summaries_show_on_cards_and_go_stale_with_the_code`
