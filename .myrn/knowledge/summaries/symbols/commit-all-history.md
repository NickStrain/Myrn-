---
id: commit-all-history
type: summary
status: unverified
title: commit_all
source: myrn summarize
about: sym:crates/myrn-engine/tests/history.rs#commit_all
anchors:
- sym:crates/myrn-engine/tests/history.rs#commit_all@51b56bd0de196688
map_version: '2'
---
# commit_all

Stages every path matching '*' with index add_all, writes the tree, and commits it to HEAD with signature t <t@example.invalid>. Returns the new commit id as a string; panics on any git error.

## Behaviour

- no existing HEAD -> creates a root commit with no parents
- existing HEAD -> the new commit's parent is the previous HEAD commit
- unchanged working tree -> still creates a commit (no change check)

## Terms

git commit, stage all, snapshot, repo commit, sha, head commit, fixture commit

## Connections

- **Location:** `crates/myrn-engine/tests/history.rs:19-38`
- **Signature:** `fn commit_all(root: &Path, msg: &str) -> String`
- **In:** [[file-crates-myrn-engine-tests-history-rs]]
- **Called by:** [[setup-history]]
- **Tests:** `diff_compares_shared_knowledge_between_points`
