---
id: rank-record-weight
type: summary
status: unverified
title: rank::record_weight
source: myrn summarize
about: sym:crates/myrn-engine/src/retrieve/rank.rs#retrieve::rank::record_weight
anchors:
- sym:crates/myrn-engine/src/retrieve/rank.rs#retrieve::rank::record_weight@bda08471407ad559
map_version: '2'
---
# rank::record_weight

Returns a score multiplier for earned records based on status and contradiction edges, or None for code nodes and episodes so they are left unchanged.

## Behaviour

- Missing node, non-earned origin, or episode type returns None; superseded status returns Some(0.0), which drops the item
- Status weights: verified 1.1, suspect 0.7, any other status 1.0
- Any 'contradicts' edge in either direction halves the weight, so suspect and contradicted gives 0.35

## Terms

superseded fact, contradicted note, verified boost, suspect demotion, record status, learned record weight, contradiction penalty

## Connections

- **Location:** `crates/myrn-engine/src/retrieve/rank.rs:139-161`
- **Signature:** `fn record_weight(c: &rusqlite::Connection, id: &str) -> Result<Option<f64>>`
- **In:** [[file-crates-myrn-engine-src-retrieve-rank-rs]]
- **Calls:** [[graph-node-graph]]
- **Called by:** [[rank-rank]]
