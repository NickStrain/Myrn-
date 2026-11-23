---
id: git-root-commit
type: summary
status: unverified
title: git::root_commit
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/git.rs#indexer::git::root_commit
anchors:
- sym:crates/myrn-engine/src/indexer/git.rs#indexer::git::root_commit@aa3337044a2b22e0
map_version: '2'
---
# git::root_commit

Returns the full SHA of the earliest commit reachable from HEAD, used to identify the project.

## Behaviour

- repo with history -> 40-char hex SHA of the first commit in reverse topological order
- unborn HEAD (no commits yet) or any libgit2 error -> None
- multiple root commits in merged history -> returns only one of them

## Terms

initial commit, first commit, project identity, genesis commit, root sha, repository origin

## Connections

- **Location:** `crates/myrn-engine/src/indexer/git.rs:16-21`
- **Signature:** `pub fn root_commit(repo: &Repository) -> Option<String>`
- **In:** [[file-crates-myrn-engine-src-indexer-git-rs]]
- **Called by:** [[api-locate]], [[engine-doctor]]
