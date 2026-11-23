---
id: sync-issue
type: summary
status: unverified
title: sync::Issue
source: myrn summarize
about: sym:crates/myrn-store/src/sync.rs#sync::Issue
anchors:
- sym:crates/myrn-store/src/sync.rs#sync::Issue@ef5322d37974c64b
map_version: '2'
---
# sync::Issue

One recorded problem for a note file, stored in note_issues and shown by status and doctor.

## Behaviour

- kind is one of 'header', 'duplicate', 'dangling' as written by link
- detail is the human-readable message, e.g. 'links -> `x` matches nothing'
- path and note_id identify the file and the note id it was reported against

## Terms

problem, warning, malformed note, dangling link, duplicate id, note error

## Connections

- **Location:** `crates/myrn-store/src/sync.rs:49-54`
- **Signature:** `pub struct Issue`
- **In:** [[file-crates-myrn-store-src-sync-rs]]
