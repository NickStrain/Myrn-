---
id: sync-summary
type: summary
status: unverified
title: sync::summary
source: myrn summarize
about: sym:crates/myrn-store/src/sync.rs#sync::summary
anchors:
- sym:crates/myrn-store/src/sync.rs#sync::summary@6c50baf327daad6d
map_version: '2'
---
# sync::summary

Returns the counts of private and shared notes and of header, dangling and duplicate issues.

## Behaviour

- an empty database gives all zeros
- malformed counts distinct paths, not rows
- counts come straight from the notes and note_issues tables

## Terms

counts, note stats, health, problem totals

## Connections

- **Location:** `crates/myrn-store/src/sync.rs:727-736`
- **Signature:** `pub fn summary(conn: &Connection) -> Result<NotesSummary>`
- **In:** [[file-crates-myrn-store-src-sync-rs]]
- **Calls:** [[graph-count]]
- **Called by:** [[engine-status]], [[engine-doctor]]
