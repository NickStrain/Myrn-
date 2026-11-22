---
id: rank-record-weight
type: summary
status: unverified
title: rank::record_weight
source: myrn summarize
about: sym:crates/myrn-engine/src/retrieve/rank.rs#retrieve::rank::record_weight
anchors:
- sym:crates/myrn-engine/src/retrieve/rank.rs#retrieve::rank::record_weight@bda08471407ad559
---
# rank::record_weight

Returns a multiplier for earned, non-episode records: superseded gives 0 (so the record is dropped), verified 1.1, suspect 0.7, other statuses 1.0, and any contradicts edge in either direction halves it. Returns None for code nodes and episodes, and propagates query errors.

## Connections

- **Location:** `crates/myrn-engine/src/retrieve/rank.rs:113-135`
- **Signature:** `fn record_weight(c: &rusqlite::Connection, id: &str) -> Result<Option<f64>>`
- **In:** [[file-crates-myrn-engine-src-retrieve-rank-rs]]
- **Calls:** [[graph-node-graph]]
- **Called by:** [[rank-rank]]
