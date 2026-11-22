---
id: sync-issues
type: summary
status: unverified
title: sync::issues
source: myrn summarize
about: sym:crates/myrn-store/src/sync.rs#sync::issues
anchors:
- sym:crates/myrn-store/src/sync.rs#sync::issues@335d7bf59a5f7b22
---
# sync::issues

Returns up to limit rows from note_issues, with header issues first and then ordered by path.

## Connections

- **Location:** `crates/myrn-store/src/sync.rs:521-535`
- **Signature:** `pub fn issues(conn: &Connection, limit: usize) -> Result<Vec<Issue>>`
- **In:** [[file-crates-myrn-store-src-sync-rs]]
- **Called by:** [[engine-note-issues]]
