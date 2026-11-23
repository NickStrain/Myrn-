---
id: repo-58c6
type: summary
status: unverified
title: repo
source: myrn summarize
about: sym:crates/myrn-engine/tests/records.rs#repo
anchors:
- sym:crates/myrn-engine/tests/records.rs#repo@a8b6bd3fb8396fba
map_version: '2'
---
# repo

Creates a temp directory with the PY package, commits it to a new git repo as 'fix unhashable items', and initialises an Engine there with home under the temp dir. Returns the bundled Repo.

## Behaviour

- no input -> Repo whose single commit contains every PY file
- Engine::init second return value is discarded
- any git, IO or engine failure -> panic via unwrap

## Terms

fixture repo, test repo setup, initial commit, git init, engine init, temp repo

## Connections

- **Location:** `crates/myrn-engine/tests/records.rs:50-72`
- **Signature:** `fn repo() -> Repo`
- **In:** [[file-crates-myrn-engine-tests-records-rs]]
- **Calls:** [[engine-init]], [[write-records]]
- **Tests:** `rejections_say_what_to_fix`, `bugs_are_stored_deduplicated_and_recalled`, `contradictions_are_flagged_and_superseded_records_drop_out`, `summaries_show_on_cards_and_go_stale_with_the_code`, `session_end_asks_once_per_batch_of_edits`, `shell_edits_are_captured_and_lead_to_a_record_request`
