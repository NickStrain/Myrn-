---
id: sync-noteroot
type: summary
status: unverified
title: sync::NoteRoot
source: myrn summarize
about: sym:crates/myrn-store/src/sync.rs#sync::NoteRoot
anchors:
- sym:crates/myrn-store/src/sync.rs#sync::NoteRoot@fc28a164bfa3e983
map_version: '2'
---
# sync::NoteRoot

Describes one notes directory: its path, its visibility, and the label used as the prefix of file keys shown to people.

## Behaviour

- label 'notes' with file notes/sub/a.md gives key 'notes/sub/a.md'
- visibility is copied onto every file found under dir
- dir is walked as given; nothing is created or checked for existence here

## Terms

notes directory, knowledge dir, private notes, shared dir, visibility, label, root

## Connections

- **Location:** `crates/myrn-store/src/sync.rs:30-35`
- **Signature:** `pub struct NoteRoot`
- **In:** [[file-crates-myrn-store-src-sync-rs]]
