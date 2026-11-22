---
id: file-crates-myrn-store-src-episodes-rs
type: summary
status: unverified
title: crates/myrn-store/src/episodes.rs
source: myrn summarize
about: file:crates/myrn-store/src/episodes.rs
anchors:
- file:crates/myrn-store/src/episodes.rs@cd902e38ca39209a
---
# crates/myrn-store/src/episodes.rs

Append-only per-session event logs in the private store's episodes/ directory, one JSON object per line in <session>.jsonl files. Defines the Entry record and the functions that build log paths, append entries, read logs back, and list sessions by recency.

## Connections

- **Location:** `crates/myrn-store/src/episodes.rs`
- **In:** [[dir-crates-myrn-store-src]]
- **Defines:** [[episodes-entry]], [[episodes-file-name]], [[episodes-path-for]], [[episodes-append]], [[episodes-read]], [[episodes-list]]
- **Imported by:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-engine-src-capture-episode-rs]], [[file-crates-myrn-engine-src-capture-mod-rs]], [[file-crates-myrn-engine-src-distill-record-rs]], [[file-crates-myrn-engine-tests-records-rs]], [[file-crates-myrn-store-src-lib-rs]]
