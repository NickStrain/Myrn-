---
id: repo-58c6
type: summary
status: unverified
title: repo
source: myrn summarize
about: sym:crates/myrn-engine/tests/records.rs#repo
anchors:
- sym:crates/myrn-engine/tests/records.rs#repo@a8b6bd3fb8396fba
---
# repo

Builds the fixture by writing PY into a temp dir, git-initializing it, committing everything as 'fix unhashable items', and calling Engine::init with a temp home directory.

## Connections

- **Location:** `crates/myrn-engine/tests/records.rs:50-72`
- **Signature:** `fn repo() -> Repo`
- **In:** [[file-crates-myrn-engine-tests-records-rs]]
- **Calls:** [[engine-init]], [[write-records]]
- **Tests:** `rejections_say_what_to_fix`, `bugs_are_stored_deduplicated_and_recalled`, `contradictions_are_flagged_and_superseded_records_drop_out`, `summaries_show_on_cards_and_go_stale_with_the_code`, `session_end_asks_once_per_batch_of_edits`, `shell_edits_are_captured_and_lead_to_a_record_request`
