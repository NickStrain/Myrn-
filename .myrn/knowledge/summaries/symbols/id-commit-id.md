---
id: id-commit-id
type: summary
status: unverified
title: id::commit_id
source: myrn summarize
about: sym:crates/myrn-model/src/id.rs#id::commit_id
anchors:
- sym:crates/myrn-model/src/id.rs#id::commit_id@52dd6398004efd90
map_version: '2'
---
# id::commit_id

Builds a commit node id by prefixing a SHA with `commit:`.

## Behaviour

- "abc123" -> "commit:abc123"
- Does not check the SHA format or length; any string is accepted

## Terms

git commit, sha, revision id, commit hash

## Connections

- **Location:** `crates/myrn-model/src/id.rs:32-34`
- **Signature:** `pub fn commit_id(sha: &str) -> String`
- **In:** [[file-crates-myrn-model-src-id-rs]]
- **Called by:** [[engine-history]], [[git-index-history]], [[git-index-commit]]
