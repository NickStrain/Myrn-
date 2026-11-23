---
id: knowledge-file-history
type: summary
status: unverified
title: knowledge::file_history
source: myrn summarize
about: sym:crates/myrn-engine/src/knowledge.rs#knowledge::file_history
anchors:
- sym:crates/myrn-engine/src/knowledge.rs#knowledge::file_history@8239de0cd90db818
map_version: '2'
---
# knowledge::file_history

Lists up to `limit` commits from HEAD, newest first, that added, removed or changed one file path, comparing its blob id with the first parent's.

## Behaviour

- root commit containing the file -> rel `added`
- merge commits are compared only with their first parent
- HEAD cannot be pushed -> empty list, no error

## Terms

git log, commit history, file history, changes to file, blame, touching commits

## Connections

- **Location:** `crates/myrn-engine/src/knowledge.rs:545-585`
- **Signature:** `fn file_history(repo: &git2::Repository, path: &str, limit: usize) -> Vec<TracedCommit>`
- **In:** [[file-crates-myrn-engine-src-knowledge-rs]]
- **Calls:** [[episode-summary]], [[sym-parent]]
- **Called by:** [[engine-trace]]
