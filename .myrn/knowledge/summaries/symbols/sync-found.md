---
id: sync-found
type: summary
status: unverified
title: sync::Found
source: myrn summarize
about: sym:crates/myrn-store/src/sync.rs#sync::Found
anchors:
- sym:crates/myrn-store/src/sync.rs#sync::Found@8e11fb40083c0259
map_version: '2'
---
# sync::Found

One note file seen by the walk, with its key, absolute path, top-level folder, file stem, visibility, size and modification time in nanoseconds.

## Behaviour

- folder is the first path segment for nested files and None for top-level files
- stem is the file name without the '.md' suffix
- mtime is 0 when the modification time cannot be read

## Terms

scanned file, file entry, stat record, walk result, file metadata

## Connections

- **Location:** `crates/myrn-store/src/sync.rs:68-76`
- **Signature:** `struct Found`
- **In:** [[file-crates-myrn-store-src-sync-rs]]
