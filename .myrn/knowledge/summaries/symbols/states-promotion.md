---
id: states-promotion
type: summary
status: unverified
title: states::Promotion
source: myrn summarize
about: sym:crates/myrn-engine/src/validity/states.rs#validity::states::Promotion
anchors:
- sym:crates/myrn-engine/src/validity/states.rs#validity::states::Promotion@7eb59ff9cc816c11
map_version: '2'
---
# states::Promotion

Holds the id of a record that became verified and the reason string that justified it.

## Behaviour

- Returned by promote, one per record moved to verified or locally verified
- reason reads like: cited test `X` passed, or fixing commit ... is in the history

## Terms

verified, promotion, earned verification, reason

## Connections

- **Location:** `crates/myrn-engine/src/validity/states.rs:25-28`
- **Signature:** `pub struct Promotion`
- **In:** [[file-crates-myrn-engine-src-validity-states-rs]]
