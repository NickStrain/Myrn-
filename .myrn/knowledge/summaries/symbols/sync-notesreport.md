---
id: sync-notesreport
type: summary
status: unverified
title: sync::NotesReport
source: myrn summarize
about: sym:crates/myrn-store/src/sync.rs#sync::NotesReport
anchors:
- sym:crates/myrn-store/src/sync.rs#sync::NotesReport@0f5f86cc083d2796
map_version: '2'
---
# sync::NotesReport

Result of one sync: how many note files were found, how many were re-parsed, how many rows were removed, and whether the link pass ran.

## Behaviour

- indexed counts only files re-parsed and stored, not files skipped by hash
- relinked is true when relink was requested, notes_linked was missing, or indexed or removed is above zero
- notes is the count of all .md files found, including unchanged ones

## Terms

sync result, scan report, counts, relinked, indexed count, removed count

## Connections

- **Location:** `crates/myrn-store/src/sync.rs:38-46`
- **Signature:** `pub struct NotesReport`
- **In:** [[file-crates-myrn-store-src-sync-rs]]
