---
id: file-crates-myrn-engine-src-retrieve-seed-rs
type: summary
status: unverified
title: crates/myrn-engine/src/retrieve/seed.rs
source: myrn summarize
about: file:crates/myrn-engine/src/retrieve/seed.rs
anchors:
- file:crates/myrn-engine/src/retrieve/seed.rs@c2af5f5179afb3ae
---
# crates/myrn-engine/src/retrieve/seed.rs

Finds starting nodes for a task or query: named symbols, file paths, path:line locations, and full-text matches over names, qualified-name words, docs and summaries. Main parts are find (one query to ranked nodes), seeds (free-text task to scored Seed list), and helpers that split task text into words and code tokens and detect bug reports.

## Connections

- **Location:** `crates/myrn-engine/src/retrieve/seed.rs`
- **In:** [[dir-crates-myrn-engine-src-retrieve]]
- **Defines:** [[seed-find]], [[seed-is-stub]], [[seed-drop-shadowed-stubs]], [[seed-myrn-index-lang]], [[seed-segments]], [[seed-ends-with-segments]], [[seed-seed]], [[seed-task-words]], [[seed-backticked-spans]], [[seed-code-tokens]], [[seed-backticked]], [[seed-seeds]], [[seed-asks-about-bug]], [[seed-is-question]]
- **Imports:** [[file-crates-myrn-store-src-fts-rs]], [[file-crates-myrn-store-src-graph-rs]], [[file-crates-myrn-store-src-lib-rs]]
- **Imported by:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-engine-src-embeddings-rs]], [[file-crates-myrn-engine-src-retrieve-mod-rs]], [[file-crates-myrn-engine-src-retrieve-rank-rs]]
