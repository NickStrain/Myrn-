---
id: indexed-repo
type: summary
status: unverified
title: indexed_repo
source: myrn summarize
about: sym:crates/myrn-engine/tests/notes.rs#indexed_repo
anchors:
- sym:crates/myrn-engine/tests/notes.rs#indexed_repo@f65d2a799b71a7ad
---
# indexed_repo

Builds the repo, copies the fixtures with the first 6 characters of the unique_everseen body hash as BODY_HASH, runs refresh, asserts 5 notes were indexed and none failed, and returns the Repo.

## Connections

- **Location:** `crates/myrn-engine/tests/notes.rs:135-143`
- **Signature:** `fn indexed_repo() -> Repo`
- **In:** [[file-crates-myrn-engine-tests-notes-rs]]
- **Calls:** [[engine-refresh]], [[repo-notes]], [[copy-fixtures]], [[body-hash]], [[layout-notes-dir]]
- **Tests:** `notes_become_nodes_edges_anchors_and_summaries`, `malformed_notes_and_dangling_links_are_flagged_not_dropped`, `only_changed_notes_are_reindexed_including_hand_edits`, `links_and_summaries_survive_code_changes`, `rebuild_recreates_the_same_index_from_code_and_notes`
