---
id: fact-validity
type: summary
status: unverified
title: fact
source: myrn summarize
about: sym:crates/myrn-engine/tests/validity.rs#fact
anchors:
- sym:crates/myrn-engine/tests/validity.rs#fact@28455bb3cc67c40b
map_version: '2'
---
# fact

Records a fact about a symbol through record_json with the symbol as evidence, asserts it was accepted, and returns the first new id.

## Behaviour

- body + about 'unique_everseen' -> fact with evidence [{symbol: unique_everseen}], returns its id
- record rejected -> panic showing the rendered outcome
- only ids[0] is returned, even if more ids were created

## Terms

record fact, add fact, create note, record note, assert accepted, record json helper

## Connections

- **Location:** `crates/myrn-engine/tests/validity.rs:85-93`
- **Signature:** `fn fact(e: &mut Engine, body: &str, about: &str) -> String`
- **In:** [[file-crates-myrn-engine-tests-validity-rs]]
- **Calls:** [[engine-record-json]]
- **Tests:** `branch_switch_makes_records_suspect_and_valid_again`, `reformat_or_comment_only_changes_have_no_effect`, `records_from_an_unmerged_branch_apply_only_where_that_code_exists`, `worktrees_share_notes_but_judge_them_by_their_own_files`, `shared_records_from_a_teammate_apply_when_their_code_matches`, `suspect_records_are_labelled_ranked_down_and_shown_where_the_agent_works`
