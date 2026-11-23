---
id: sync-conflict
type: summary
status: unverified
title: sync::Conflict
source: myrn summarize
about: sym:crates/myrn-store/src/sync.rs#sync::Conflict
anchors:
- sym:crates/myrn-store/src/sync.rs#sync::Conflict@7b38e5d6e9d93487
map_version: '2'
---
# sync::Conflict

A pair of records that disagree, with their titles and visibilities, for a person to resolve.

## Behaviour

- a and b are the record ids in the order the edge stores them
- a_visibility and b_visibility come from the stored extra JSON and are empty when absent
- one Conflict per pair, regardless of edge direction

## Terms

disagreement, conflicting records, clash, contradiction pair

## Connections

- **Location:** `crates/myrn-store/src/sync.rs:606-613`
- **Signature:** `pub struct Conflict`
- **In:** [[file-crates-myrn-store-src-sync-rs]]
