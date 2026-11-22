---
id: walk-rec
type: summary
status: unverified
title: walk::rec
source: myrn summarize
about: sym:crates/myrn-store/src/sync.rs#sync::walk::rec
anchors:
- sym:crates/myrn-store/src/sync.rs#sync::walk::rec@ce70f5022c34a8a1
---
# walk::rec

Recursive directory visitor used by walk; silently skips unreadable directories and entries whose metadata cannot be read.

## Connections

- **Location:** `crates/myrn-store/src/sync.rs:79-114`
- **Signature:** `fn rec(dir: &Path, rel: &str, root: &NoteRoot, out: &mut Vec<Found>)`
- **In:** [[file-crates-myrn-store-src-sync-rs]]
- **Calls:** [[walk-rec]]
- **Called by:** [[sync-walk]], [[walk-rec]]
