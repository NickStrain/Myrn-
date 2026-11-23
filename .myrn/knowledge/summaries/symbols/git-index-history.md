---
id: git-index-history
type: summary
status: unverified
title: git::index_history
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/git.rs#indexer::git::index_history
anchors:
- sym:crates/myrn-engine/src/indexer/git.rs#indexer::git::index_history@978e12c666c77996
map_version: '2'
---
# git::index_history

Walks up to `depth` commits from HEAD newest-first, indexes each unindexed one in a single transaction, and records HEAD in meta key git_head; returns how many commits were added.

## Behaviour

- root not inside a git repo, or no HEAD -> Ok(0) without touching the DB
- meta git_head equals current HEAD -> Ok(0), no walk
- walk stops at the first commit whose node already exists, so older history is not re-scanned

## Terms

backfill, sync git history, history import, incremental indexing, commit walk, catch up history

## Connections

- **Location:** `crates/myrn-engine/src/indexer/git.rs:32-70`
- **Signature:** `pub fn index_history(conn: &Connection, root: &Path, depth: usize) -> Result<usize>`
- **In:** [[file-crates-myrn-engine-src-indexer-git-rs]]
- **Calls:** [[git-head-sha]], [[git-index-commit]], [[fileparser-new]], [[id-commit-id]], [[graph-node-graph]]
- **Called by:** [[indexer-refresh]]
