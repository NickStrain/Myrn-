---
id: git-head-sha
type: summary
status: unverified
title: git::head_sha
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/git.rs#indexer::git::head_sha
anchors:
- sym:crates/myrn-engine/src/indexer/git.rs#indexer::git::head_sha@5ab293dcbf16922e
map_version: '2'
---
# git::head_sha

Returns the full SHA of the commit HEAD points to, peeling symbolic refs and tags to a commit.

## Behaviour

- checked-out branch with commits -> 40-char hex SHA of its tip
- HEAD on an unborn branch or not resolvable to a commit -> None
- any libgit2 error -> None, never an error

## Terms

HEAD, current commit, checked-out revision, tip sha, head revision, branch tip

## Connections

- **Location:** `crates/myrn-engine/src/indexer/git.rs:23-29`
- **Signature:** `pub fn head_sha(repo: &Repository) -> Option<String>`
- **In:** [[file-crates-myrn-engine-src-indexer-git-rs]]
- **Called by:** [[git-index-history]]
