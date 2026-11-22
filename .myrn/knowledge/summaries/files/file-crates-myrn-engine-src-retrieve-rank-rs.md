---
id: file-crates-myrn-engine-src-retrieve-rank-rs
type: summary
status: unverified
title: crates/myrn-engine/src/retrieve/rank.rs
source: myrn summarize
about: file:crates/myrn-engine/src/retrieve/rank.rs
anchors:
- file:crates/myrn-engine/src/retrieve/rank.rs@58721f9b66f5e6ae
---
# crates/myrn-engine/src/retrieve/rank.rs

Expands seed nodes one hop along the graph (calls in both directions, direct tests, about/located_in records and their caused_by bugs, and same-file symbols for file seeds), then scores and sorts the candidates. Holds the MIN_SCORE gate (3.0) and record_weight, which applies status and contradiction multipliers to learned records.

## Connections

- **Location:** `crates/myrn-engine/src/retrieve/rank.rs`
- **In:** [[dir-crates-myrn-engine-src-retrieve]]
- **Defines:** [[rank-ranked]], [[rank-rank]], [[rank-record-weight]]
- **Imports:** [[file-crates-myrn-engine-src-retrieve-seed-rs]], [[file-crates-myrn-engine-src-retrieve-walks-rs]], [[file-crates-myrn-store-src-graph-rs]], [[file-crates-myrn-store-src-lib-rs]]
- **Imported by:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-engine-src-retrieve-mod-rs]]
