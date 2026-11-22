---
id: git-root-commit
type: summary
status: unverified
title: git::root_commit
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/git.rs#indexer::git::root_commit
anchors:
- sym:crates/myrn-engine/src/indexer/git.rs#indexer::git::root_commit@aa3337044a2b22e0
---
# git::root_commit

Returns the SHA of the first commit in a reverse topological walk from HEAD, or None if HEAD is unreadable or the repo has no commits. With multiple root commits, the result is whichever one the walk yields first.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/git.rs:16-21`
- **Signature:** `pub fn root_commit(repo: &Repository) -> Option<String>`
- **In:** [[file-crates-myrn-engine-src-indexer-git-rs]]
- **Called by:** [[api-locate]], [[engine-doctor]]
