---
id: sync-drift-sync
type: summary
status: unverified
title: sync::drift
source: myrn summarize
about: sym:crates/myrn-store/src/sync.rs#sync::drift
anchors:
- sym:crates/myrn-store/src/sync.rs#sync::drift@82ce5e836544bbc2
map_version: '2'
---
# sync::drift

Returns the anchors that no longer match the current code, for one record when note_id is given, otherwise for all earned records.

## Behaviour

- note_id None returns drift for every record, ordered by note_id
- anchors with an empty or NULL body hash are not reported
- a symbol whose current hash starts with the anchored hash counts as a match

## Terms

stale anchors, anchor check, outdated records, anchor drift report

## Connections

- **Location:** `crates/myrn-store/src/sync.rs:705-725`
- **Signature:** `pub fn drift(conn: &Connection, note_id: Option<&str>) -> Result<Vec<Drift>>`
- **In:** [[file-crates-myrn-store-src-sync-rs]]
- **Called by:** [[engine-trace]], [[engine-suspect-notice]], [[engine-suspects]]
