---
id: walk-walk
type: summary
status: unverified
title: walk::walk
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/walk.rs#indexer::walk::walk
anchors:
- sym:crates/myrn-engine/src/indexer/walk.rs#indexer::walk::walk@74206f607ab372a5
map_version: '2'
---
# walk::walk

Traverses root with standard gitignore filters plus the extra ignore matcher, returns supported-language files under max_bytes, sorted by path. Symlinks are not followed and read errors are skipped silently.

## Behaviour

- ignore_file missing -> only standard gitignore filters apply (require_git is false, so .gitignore works without a git repo)
- file size equal to max_bytes is kept; size greater than max_bytes is skipped
- file outside root, unsupported extension (Lang::from_path returns None), or unreadable metadata -> skipped; returns Ok with the rest

## Terms

file discovery, directory traversal, source files, gitignore filter, size limit, max bytes, skip large files, symlinks, sorted file list, repo scan

## Connections

- **Location:** `crates/myrn-engine/src/indexer/walk.rs:32-80`
- **Signature:** `pub fn walk(root: &Path, ignore_file: &Path, max_bytes: u64) -> Result<Vec<FileStat>>`
- **In:** [[file-crates-myrn-engine-src-indexer-walk-rs]]
- **Calls:** [[lang-from-path]], [[walk-load-ignore]], [[paths-repo-rel]]
- **Called by:** [[indexer-refresh]]
