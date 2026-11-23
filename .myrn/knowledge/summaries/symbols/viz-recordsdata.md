---
id: viz-recordsdata
type: summary
status: unverified
title: viz::RecordsData
source: myrn summarize
about: sym:crates/myrn-engine/src/viz.rs#viz::RecordsData
anchors:
- sym:crates/myrn-engine/src/viz.rs#viz::RecordsData@71ff6513a5a0f2a2
map_version: '2'
---
# viz::RecordsData

Payload for the records view: learned records with usage counts, and the sync conflicts.

## Behaviour

- records whose source equals summarize::SOURCE are dropped
- conflicts come from sync::conflicts on the same connection

## Terms

learned records, sync conflicts, record list, record payload

## Connections

- **Location:** `crates/myrn-engine/src/viz.rs:109-112`
- **Signature:** `pub struct RecordsData`
- **In:** [[file-crates-myrn-engine-src-viz-rs]]
