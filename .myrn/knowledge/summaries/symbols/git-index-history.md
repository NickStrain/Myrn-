---
id: git-index-history
type: summary
status: unverified
title: git::index_history
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/git.rs#indexer::git::index_history
anchors:
- sym:crates/myrn-engine/src/indexer/git.rs#indexer::git::index_history@978e12c666c77996
---
# git::index_history

Discovers the repo from root and indexes up to depth commits from HEAD, newest first by time, skipping any commit already in the graph and stopping at the first one found there. Returns the number added, or 0 without error if the repo is missing, HEAD is unborn, or meta 'git_head' already equals HEAD; 'git_head' is only updated in the same transaction after all commits are indexed.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/git.rs:32-70`
- **Signature:** `pub fn index_history(conn: &Connection, root: &Path, depth: usize) -> Result<usize>`
- **In:** [[file-crates-myrn-engine-src-indexer-git-rs]]
- **Calls:** [[git-head-sha]], [[git-index-commit]], [[fileparser-new]], [[id-commit-id]], [[graph-node-graph]]
- **Called by:** [[indexer-refresh]]
