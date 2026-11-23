---
id: walk-rec
type: summary
status: unverified
title: walk::rec
source: myrn summarize
about: sym:crates/myrn-store/src/sync.rs#sync::walk::rec
anchors:
- sym:crates/myrn-store/src/sync.rs#sync::walk::rec@ce70f5022c34a8a1
map_version: '2'
---
# walk::rec

Recursive helper of walk that visits one directory and pushes a Found for each .md file in it and its subdirectories.

## Behaviour

- entries whose metadata cannot be read are skipped
- rel is empty at the root and grows with '/' separators for subdirectories
- a file that ends in '.md' is pushed; other files are ignored

## Terms

recursion, directory walk, helper, subfolder scan

## Connections

- **Location:** `crates/myrn-store/src/sync.rs:79-114`
- **Signature:** `fn rec(dir: &Path, rel: &str, root: &NoteRoot, out: &mut Vec<Found>)`
- **In:** [[file-crates-myrn-store-src-sync-rs]]
- **Calls:** [[walk-rec]]
- **Called by:** [[sync-walk]], [[walk-rec]]
