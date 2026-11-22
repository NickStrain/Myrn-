---
id: file-crates-myrn-engine-src-retrieve-seed-rs
type: summary
status: unverified
title: crates/myrn-engine/src/retrieve/seed.rs
source: myrn summarize
about: file:crates/myrn-engine/src/retrieve/seed.rs
anchors:
- file:crates/myrn-engine/src/retrieve/seed.rs@3b57adf316a50462
---
# crates/myrn-engine/src/retrieve/seed.rs

Finds starting nodes for a task or query: named symbols, file paths, `path:line` locations, and full-text matches over names, qualified-name words, docs and summaries. Main entry points are `find` (a single query) and `seeds` (a free-text task), with helpers for tokenizing task text and filtering type stubs.

## Connections

- **Location:** `crates/myrn-engine/src/retrieve/seed.rs`
- **In:** [[dir-crates-myrn-engine-src-retrieve]]
- **Defines:** [[seed-find]], [[seed-is-stub]], [[seed-drop-shadowed-stubs]], [[seed-myrn-index-lang]], [[seed-segments]], [[seed-ends-with-segments]], [[seed-seed]], [[seed-task-words]], [[seed-code-tokens]], [[seed-backticked]], [[seed-seeds]]
- **Imports:** [[file-crates-myrn-store-src-fts-rs]], [[file-crates-myrn-store-src-graph-rs]], [[file-crates-myrn-store-src-lib-rs]]
- **Imported by:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-engine-src-retrieve-mod-rs]], [[file-crates-myrn-engine-src-retrieve-rank-rs]]
