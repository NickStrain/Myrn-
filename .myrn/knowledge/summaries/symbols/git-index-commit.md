---
id: git-index-commit
type: summary
status: unverified
title: git::index_commit
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/git.rs#indexer::git::index_commit
anchors:
- sym:crates/myrn-engine/src/indexer/git.rs#indexer::git::index_commit@1330331656920d41
---
# git::index_commit

Upserts a commit node (id from commit_id, title = summary, qname = 7-char short SHA, doc = message truncated to 2000 chars, extra JSON with author, time, parents, and reverts parsed from 'This reverts commit <hex>') and its FTS entry. For non-merge commits, it diffs against the first parent with rename detection and zero context, inserts commit_files rows (copies stored as added, old_path only for renames), and for added, modified, or renamed files in a language Lang::from_path recognizes, adds 'touches' edges to the innermost overlapping definitions (test definitions use test_id); merge commits get only the node.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/git.rs:72-201`
- **Signature:** `fn index_commit( conn: &Connection, repo: &Repository, oid: Oid, parser: &mut FileParser, ) -> Result<()>`
- **In:** [[file-crates-myrn-engine-src-indexer-git-rs]]
- **Calls:** [[engine-status]], [[episode-summary]], [[lang-from-path]], [[fileparser-parse]], [[sym-parent]], [[id-symbol-id]], [[id-test-id]], [[id-commit-id]], [[fts-index-node]], [[graph-upsert-node]], [[graph-insert-edge]]
- **Called by:** [[git-index-history]]
