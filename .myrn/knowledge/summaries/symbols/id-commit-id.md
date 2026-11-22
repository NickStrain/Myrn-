---
id: id-commit-id
type: summary
status: unverified
title: id::commit_id
source: myrn summarize
about: sym:crates/myrn-model/src/id.rs#id::commit_id
anchors:
- sym:crates/myrn-model/src/id.rs#id::commit_id@52dd6398004efd90
---
# id::commit_id

Takes a commit SHA and returns `commit:<sha>` as given, without checking that it is a valid SHA.

## Connections

- **Location:** `crates/myrn-model/src/id.rs:32-34`
- **Signature:** `pub fn commit_id(sha: &str) -> String`
- **In:** [[file-crates-myrn-model-src-id-rs]]
- **Called by:** [[engine-history]], [[git-index-history]], [[git-index-commit]]
