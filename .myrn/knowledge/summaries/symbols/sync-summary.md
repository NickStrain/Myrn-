---
id: sync-summary
type: summary
status: unverified
title: sync::summary
source: myrn summarize
about: sym:crates/myrn-store/src/sync.rs#sync::summary
anchors:
- sym:crates/myrn-store/src/sync.rs#sync::summary@6c50baf327daad6d
---
# sync::summary

Counts notes by visibility and issues by kind for NotesSummary. malformed counts distinct paths with header issues; dangling and duplicates count issue rows.

## Connections

- **Location:** `crates/myrn-store/src/sync.rs:510-519`
- **Signature:** `pub fn summary(conn: &Connection) -> Result<NotesSummary>`
- **In:** [[file-crates-myrn-store-src-sync-rs]]
- **Calls:** [[graph-count]]
- **Called by:** [[engine-status]], [[engine-doctor]]
