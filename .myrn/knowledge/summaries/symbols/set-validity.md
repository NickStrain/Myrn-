---
id: set-validity
type: summary
status: unverified
title: set
source: myrn summarize
about: sym:crates/myrn-engine/tests/validity.rs#set
anchors:
- sym:crates/myrn-engine/tests/validity.rs#set@bec872dd4ae99bcc
map_version: '2'
---
# set

Writes one file under the repo root and then calls refresh on the engine, to simulate editing the working tree. Does not commit.

## Behaviour

- set(r, 'pkg/recipes.py', RECIPES_CHANGED) -> file rewritten, engine refreshed
- change is uncommitted, so commit history stays the same
- refresh failure -> panic

## Terms

edit file, modify file, change file, refresh engine, simulate edit, working tree change

## Connections

- **Location:** `crates/myrn-engine/tests/validity.rs:80-83`
- **Signature:** `fn set(r: &mut Repo, path: &str, text: &str)`
- **In:** [[file-crates-myrn-engine-tests-validity-rs]]
- **Calls:** [[engine-refresh]], [[write-validity]]
- **Tests:** `branch_switch_makes_records_suspect_and_valid_again`, `reformat_or_comment_only_changes_have_no_effect`, `records_from_an_unmerged_branch_apply_only_where_that_code_exists`, `suspect_records_are_labelled_ranked_down_and_shown_where_the_agent_works`, `confirm_moves_anchors_to_the_current_code`, `supersede_replaces_and_keeps_history`
