---
id: commit-all-sharing
type: summary
status: unverified
title: commit_all
source: myrn summarize
about: sym:crates/myrn-engine/tests/sharing.rs#commit_all
anchors:
- sym:crates/myrn-engine/tests/sharing.rs#commit_all@c3ea729e8ed0cbf0
map_version: '2'
---
# commit_all

Stages all files with add_all (not update_all) and commits them to HEAD as who, using a fixed dev@example.invalid email. Parent is the current HEAD, or none if the repo has no commits yet.

## Behaviour

- empty repo -> creates a root commit with no parents
- later call -> new commit whose parent is the previous HEAD
- deleted tracked files are not staged because add_all does not remove them

## Terms

git commit, stage all, snapshot, add all, commit as author, make commit, record changes

## Connections

- **Location:** `crates/myrn-engine/tests/sharing.rs:30-48`
- **Signature:** `fn commit_all(root: &Path, who: &str, msg: &str)`
- **In:** [[file-crates-myrn-engine-tests-sharing-rs]]
- **Called by:** [[two-clones]]
- **Tests:** `published_record_reaches_a_teammate_through_git`, `imported_records_start_unverified_whatever_they_say`, `shared_records_that_disagree_are_flagged_for_a_person`, `a_teammate_verifies_a_shared_record_locally_without_changing_the_file`
