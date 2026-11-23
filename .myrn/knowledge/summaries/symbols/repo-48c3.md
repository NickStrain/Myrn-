---
id: repo-48c3
type: summary
status: unverified
title: repo
source: myrn summarize
about: sym:crates/myrn-engine/tests/validity.rs#repo
anchors:
- sym:crates/myrn-engine/tests/validity.rs#repo@cdd3440cc9fecc37
map_version: '2'
---
# repo

Builds a fresh test repository: writes files() into a temp repo, git-inits it, commits it as 'first', and creates an Engine with its home under the same temp dir.

## Behaviour

- no args -> Repo whose head is the 'first' commit id
- creates tmp/repo and tmp/home
- Engine::init second return value is discarded

## Terms

test repository setup, fixture repo, init temp repo, engine init, first commit, base fixture

## Connections

- **Location:** `crates/myrn-engine/tests/validity.rs:63-71`
- **Signature:** `fn repo() -> Repo`
- **In:** [[file-crates-myrn-engine-tests-validity-rs]]
- **Calls:** [[engine-init]], [[files-validity]], [[write-validity]], [[commit-all-validity]]
- **Tests:** `branch_switch_makes_records_suspect_and_valid_again`, `reformat_or_comment_only_changes_have_no_effect`, `records_from_an_unmerged_branch_apply_only_where_that_code_exists`, `worktrees_share_notes_but_judge_them_by_their_own_files`, `shared_records_from_a_teammate_apply_when_their_code_matches`, `suspect_records_are_labelled_ranked_down_and_shown_where_the_agent_works`
