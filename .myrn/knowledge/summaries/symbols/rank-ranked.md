---
id: rank-ranked
type: summary
status: unverified
title: rank::Ranked
source: myrn summarize
about: sym:crates/myrn-engine/src/retrieve/rank.rs#retrieve::rank::Ranked
anchors:
- sym:crates/myrn-engine/src/retrieve/rank.rs#retrieve::rank::Ranked@588143517fca147a
map_version: '2'
---
# rank::Ranked

Plain record of one ranked candidate: node id, accumulated score, and the static reason string shown to the user.

## Behaviour

- Built only inside rank's accumulator, starting with score 0.0 before the first contribution is merged
- why is a &'static str, so it must be one of the literal reason strings used in rank

## Terms

retrieval result, scored candidate, injected item, match reason, why text

## Connections

- **Location:** `crates/myrn-engine/src/retrieve/rank.rs:17-21`
- **Signature:** `pub struct Ranked`
- **In:** [[file-crates-myrn-engine-src-retrieve-rank-rs]]
