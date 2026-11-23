---
id: commit-all-validity
type: summary
status: unverified
title: commit_all
source: myrn summarize
about: sym:crates/myrn-engine/tests/validity.rs#commit_all
anchors:
- sym:crates/myrn-engine/tests/validity.rs#commit_all@51b56bd0de196688
map_version: '2'
---
# commit_all

Stages every file in the working tree and commits it to HEAD with the given message, using a fixed author 't', and returns the new commit id as a string.

## Behaviour

- no HEAD yet -> commit created with no parent
- existing HEAD -> new commit's parent is the current HEAD commit
- msg 'fix x' -> commit message 'fix x', returned id is the 40-char hex hash

## Terms

git commit, stage all, snapshot working tree, commit changes, libgit2 commit, index add

## Connections

- **Location:** `crates/myrn-engine/tests/validity.rs:42-61`
- **Signature:** `fn commit_all(root: &Path, msg: &str) -> String`
- **In:** [[file-crates-myrn-engine-tests-validity-rs]]
- **Called by:** [[repo-48c3]]
- **Tests:** `shared_records_from_a_teammate_apply_when_their_code_matches`, `a_reverted_fix_does_not_verify`
