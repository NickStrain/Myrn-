---
id: walk-walk
type: summary
status: unverified
title: walk::walk
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/walk.rs#indexer::walk::walk
anchors:
- sym:crates/myrn-engine/src/indexer/walk.rs#indexer::walk::walk@74206f607ab372a5
---
# walk::walk

Returns all regular files under `root` that map to a supported `Lang`, are not ignored by standard git filters or the extra ignore file, and are at most `max_bytes`, sorted by path; unreadable entries and files outside the repo are silently skipped, and symlinks are not followed.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/walk.rs:32-80`
- **Signature:** `pub fn walk(root: &Path, ignore_file: &Path, max_bytes: u64) -> Result<Vec<FileStat>>`
- **In:** [[file-crates-myrn-engine-src-indexer-walk-rs]]
- **Calls:** [[lang-from-path]], [[walk-load-ignore]], [[paths-repo-rel]]
- **Called by:** [[indexer-refresh]]
