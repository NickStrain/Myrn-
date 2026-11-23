---
id: git-index-commit
type: summary
status: unverified
title: git::index_commit
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/git.rs#indexer::git::index_commit
anchors:
- sym:crates/myrn-engine/src/indexer/git.rs#indexer::git::index_commit@1330331656920d41
map_version: '2'
---
# git::index_commit

Writes one commit node with its FTS entry, then records each changed file and adds 'touches' edges from the commit to the innermost changed symbols in supported languages.

## Behaviour

- merge commit (more than one parent) -> node and FTS written, no file rows or touches edges
- root commit -> diffed against the empty tree, so all its files show as added
- file with no hunks, unsupported language, deleted file, or unparseable blob -> file row kept but no touches edges; a parent-less or deleted-only change is not parsed

## Terms

commit node, file change rows, touched symbols, revert detection, rename tracking, change attribution, hunk mapping

## Connections

- **Location:** `crates/myrn-engine/src/indexer/git.rs:72-201`
- **Signature:** `fn index_commit( conn: &Connection, repo: &Repository, oid: Oid, parser: &mut FileParser, ) -> Result<()>`
- **In:** [[file-crates-myrn-engine-src-indexer-git-rs]]
- **Calls:** [[engine-status]], [[episode-summary]], [[lang-from-path]], [[fileparser-parse]], [[sym-parent]], [[id-symbol-id]], [[id-test-id]], [[id-commit-id]], [[fts-index-node]], [[graph-upsert-node]], [[graph-insert-edge]]
- **Called by:** [[git-index-history]]
