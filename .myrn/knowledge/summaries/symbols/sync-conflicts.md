---
id: sync-conflicts
type: summary
status: unverified
title: sync::conflicts
source: myrn summarize
about: sym:crates/myrn-store/src/sync.rs#sync::conflicts
anchors:
- sym:crates/myrn-store/src/sync.rs#sync::conflicts@0427ae9ac684e299
map_version: '2'
---
# sync::conflicts

Lists the conflicting record pairs that have a contradicts edge, at least one shared, neither superseded.

## Behaviour

- private-only contradictions are not listed
- a pair linked in both directions appears once
- results are ordered by a id then b id before deduplication

## Terms

list conflicts, disagreements, contradictions, conflict report

## Connections

- **Location:** `crates/myrn-store/src/sync.rs:615-649`
- **Signature:** `pub fn conflicts(conn: &Connection) -> Result<Vec<Conflict>>`
- **In:** [[file-crates-myrn-store-src-sync-rs]]
- **Called by:** [[engine-status]], [[engine-viz-records]]
