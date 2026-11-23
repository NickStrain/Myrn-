---
id: status-validity
type: summary
status: unverified
title: status
source: myrn summarize
about: sym:crates/myrn-engine/tests/validity.rs#status
anchors:
- sym:crates/myrn-engine/tests/validity.rs#status@80833257facc0e48
map_version: '2'
---
# status

Reads the status column of the node with the given id directly from the engine's database and returns it as a string. Panics if no such node exists.

## Behaviour

- id of a recorded fact -> 'unverified', 'suspect', 'verified' or 'superseded'
- unknown id -> panic from unwrap, no row returned
- does not refresh the engine first, so it reads the last stored state

## Terms

record status, node status, lookup status, read status, database query, validity state

## Connections

- **Location:** `crates/myrn-engine/tests/validity.rs:73-78`
- **Signature:** `fn status(e: &Engine, id: &str) -> String`
- **In:** [[file-crates-myrn-engine-tests-validity-rs]]
- **Calls:** [[engine-db]]
- **Tests:** `branch_switch_makes_records_suspect_and_valid_again`, `reformat_or_comment_only_changes_have_no_effect`, `records_from_an_unmerged_branch_apply_only_where_that_code_exists`, `worktrees_share_notes_but_judge_them_by_their_own_files`, `shared_records_from_a_teammate_apply_when_their_code_matches`, `confirm_moves_anchors_to_the_current_code`
