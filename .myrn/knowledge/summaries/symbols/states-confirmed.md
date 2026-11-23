---
id: states-confirmed
type: summary
status: unverified
title: states::Confirmed
source: myrn summarize
about: sym:crates/myrn-engine/src/validity/states.rs#validity::states::Confirmed
anchors:
- sym:crates/myrn-engine/src/validity/states.rs#validity::states::Confirmed@e10e286b9cee787b
map_version: '2'
---
# states::Confirmed

Result of confirm: the record id, its status after resync, and how many anchor hashes were refreshed.

## Behaviour

- No anchor hash differs from the current body hash -> anchors_refreshed 0
- Status is read back from the graph after sync, empty string if absent

## Terms

confirmation, re-anchor, refreshed anchors, accepted record

## Connections

- **Location:** `crates/myrn-engine/src/validity/states.rs:49-53`
- **Signature:** `pub struct Confirmed`
- **In:** [[file-crates-myrn-engine-src-validity-states-rs]]
