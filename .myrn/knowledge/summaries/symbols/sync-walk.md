---
id: sync-walk
type: summary
status: unverified
title: sync::walk
source: myrn summarize
about: sym:crates/myrn-store/src/sync.rs#sync::walk
anchors:
- sym:crates/myrn-store/src/sync.rs#sync::walk@bb825edefd95e13b
---
# sync::walk

Recursively collects every .md file under a NoteRoot into the output vec, skipping dot-prefixed entries and names ending in .tmp. It does not read file contents.

## Connections

- **Location:** `crates/myrn-store/src/sync.rs:78-116`
- **Signature:** `fn walk(root: &NoteRoot, out: &mut Vec<Found>)`
- **In:** [[file-crates-myrn-store-src-sync-rs]]
- **Calls:** [[walk-rec]]
- **Called by:** [[sync-sync]]
