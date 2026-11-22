---
id: git-head-sha
type: summary
status: unverified
title: git::head_sha
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/git.rs#indexer::git::head_sha
anchors:
- sym:crates/myrn-engine/src/indexer/git.rs#indexer::git::head_sha@5ab293dcbf16922e
---
# git::head_sha

Returns the full SHA of the commit HEAD resolves to (peeling tags and refs), or None if HEAD is unborn or does not point to a commit.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/git.rs:23-29`
- **Signature:** `pub fn head_sha(repo: &Repository) -> Option<String>`
- **In:** [[file-crates-myrn-engine-src-indexer-git-rs]]
- **Called by:** [[git-index-history]]
