---
id: file-crates-myrn-engine-src-retrieve-rank-rs
type: summary
status: unverified
title: crates/myrn-engine/src/retrieve/rank.rs
source: myrn summarize
about: file:crates/myrn-engine/src/retrieve/rank.rs
anchors:
- file:crates/myrn-engine/src/retrieve/rank.rs@6642a936c84962cb
---
# crates/myrn-engine/src/retrieve/rank.rs

Ranks retrieval candidates by expanding the best seed nodes one graph hop (calls, tests, and earned notes or bugs linked to symbols) and merging them into a sorted list, or returning nothing when the best seed scores below MIN_SCORE. It also applies learned-mode reweighting by past usefulness, note age, and record status.

## Connections

- **Location:** `crates/myrn-engine/src/retrieve/rank.rs`
- **In:** [[dir-crates-myrn-engine-src-retrieve]]
- **Defines:** [[rank-ranked]], [[rank-rank]], [[rank-learned]], [[rank-record-weight]]
- **Imports:** [[file-crates-myrn-engine-src-config-rs]], [[file-crates-myrn-engine-src-retrieve-seed-rs]], [[file-crates-myrn-engine-src-retrieve-walks-rs]], [[file-crates-myrn-store-src-graph-rs]], [[file-crates-myrn-store-src-lib-rs]]
- **Imported by:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-engine-src-retrieve-mod-rs]]
