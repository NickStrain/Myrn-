---
id: sync-notessummary
type: summary
status: unverified
title: sync::NotesSummary
source: myrn summarize
about: sym:crates/myrn-store/src/sync.rs#sync::NotesSummary
anchors:
- sym:crates/myrn-store/src/sync.rs#sync::NotesSummary@54e37a9631db9c13
map_version: '2'
---
# sync::NotesSummary

Counts for the notes health view: private and shared note rows, files with header problems, dangling problems, and duplicate problems.

## Behaviour

- malformed counts distinct file paths with 'header' issues, not issue rows
- dangling and duplicates count issue rows of those kinds
- all counts are 0 when there are no notes and no issues

## Terms

health counts, note stats, status summary, problem totals

## Connections

- **Location:** `crates/myrn-store/src/sync.rs:57-66`
- **Signature:** `pub struct NotesSummary`
- **In:** [[file-crates-myrn-store-src-sync-rs]]
