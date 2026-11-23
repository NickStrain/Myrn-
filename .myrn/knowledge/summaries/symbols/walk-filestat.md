---
id: walk-filestat
type: summary
status: unverified
title: walk::FileStat
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/walk.rs#indexer::walk::FileStat
anchors:
- sym:crates/myrn-engine/src/indexer/walk.rs#indexer::walk::FileStat@512ad47fd15a7617
map_version: '2'
---
# walk::FileStat

Record for one indexed source file: repo-relative path, absolute path, language, size in bytes, and modification time in nanoseconds since the Unix epoch.

## Behaviour

- path is repo-relative with forward slashes, produced by myrn_store::paths::repo_rel
- mtime is 0 when the modification time is unavailable or predates the epoch
- size is the metadata length in bytes, the same value walk compares against max_bytes

## Terms

file record, file stat, file metadata, repo-relative path, mtime, modification time, file size, language

## Connections

- **Location:** `crates/myrn-engine/src/indexer/walk.rs:14-21`
- **Signature:** `pub struct FileStat`
- **In:** [[file-crates-myrn-engine-src-indexer-walk-rs]]
