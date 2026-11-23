---
id: sync-row
type: summary
status: unverified
title: sync::Row
source: myrn summarize
about: sym:crates/myrn-store/src/sync.rs#sync::Row
anchors:
- sym:crates/myrn-store/src/sync.rs#sync::Row@0da99c8fb649f801
map_version: '2'
---
# sync::Row

One stored note loaded for the link pass: its path, visibility, parsed Note and its header problems.

## Behaviour

- rows whose stored JSON cannot be parsed are not included and are deleted from notes
- problems is empty when the stored problems column is NULL or unreadable
- visibility is the stored string such as 'shared' or 'private'

## Terms

parsed note row, link input, stored note

## Connections

- **Location:** `crates/myrn-store/src/sync.rs:214-219`
- **Signature:** `struct Row`
- **In:** [[file-crates-myrn-store-src-sync-rs]]
