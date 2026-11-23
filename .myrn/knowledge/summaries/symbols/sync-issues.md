---
id: sync-issues
type: summary
status: unverified
title: sync::issues
source: myrn summarize
about: sym:crates/myrn-store/src/sync.rs#sync::issues
anchors:
- sym:crates/myrn-store/src/sync.rs#sync::issues@335d7bf59a5f7b22
map_version: '2'
---
# sync::issues

Returns up to limit issue rows, header problems first, then ordered by path.

## Behaviour

- limit 0 returns an empty list
- header kind rows come before dangling and duplicate rows
- rows are returned exactly as stored in note_issues

## Terms

problem list, note problems, warnings, issue listing

## Connections

- **Location:** `crates/myrn-store/src/sync.rs:738-752`
- **Signature:** `pub fn issues(conn: &Connection, limit: usize) -> Result<Vec<Issue>>`
- **In:** [[file-crates-myrn-store-src-sync-rs]]
- **Called by:** [[engine-note-issues]]
