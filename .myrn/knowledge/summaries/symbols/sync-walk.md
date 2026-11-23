---
id: sync-walk
type: summary
status: unverified
title: sync::walk
source: myrn summarize
about: sym:crates/myrn-store/src/sync.rs#sync::walk
anchors:
- sym:crates/myrn-store/src/sync.rs#sync::walk@bb825edefd95e13b
map_version: '2'
---
# sync::walk

Collects a Found entry for every .md file under one NoteRoot into the output vector.

## Behaviour

- names starting with '.' or ending '.tmp' are skipped, including directories
- a missing or unreadable directory is skipped silently and adds nothing
- root with notes/a.md and notes/sub/b.md yields keys 'notes/a.md' and 'notes/sub/b.md'

## Terms

scan notes, find markdown files, directory traversal, ignore hidden files, collect notes

## Connections

- **Location:** `crates/myrn-store/src/sync.rs:78-116`
- **Signature:** `fn walk(root: &NoteRoot, out: &mut Vec<Found>)`
- **In:** [[file-crates-myrn-store-src-sync-rs]]
- **Calls:** [[walk-rec]]
- **Called by:** [[sync-sync]]
