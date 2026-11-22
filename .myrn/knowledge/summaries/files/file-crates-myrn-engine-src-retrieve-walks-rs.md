---
id: file-crates-myrn-engine-src-retrieve-walks-rs
type: summary
status: unverified
title: crates/myrn-engine/src/retrieve/walks.rs
source: myrn summarize
about: file:crates/myrn-engine/src/retrieve/walks.rs
anchors:
- file:crates/myrn-engine/src/retrieve/walks.rs@5f92b46486ea4041
---
# crates/myrn-engine/src/retrieve/walks.rs

Graph traversal helpers for the retrieve layer: finds a type's direct members, a symbol's direct tests, and performs a bounded breadth-first walk over incoming `calls` edges to build impact reach (callers and tests). Also provides an in-degree count of non-test callers used as a centrality measure.

## Connections

- **Location:** `crates/myrn-engine/src/retrieve/walks.rs`
- **In:** [[dir-crates-myrn-engine-src-retrieve]]
- **Defines:** [[walks-members]], [[walks-direct-tests]], [[walks-reach]], [[walks-callers]], [[walks-in-degree]]
- **Imports:** [[file-crates-myrn-store-src-graph-rs]]
- **Imported by:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-engine-src-retrieve-mod-rs]], [[file-crates-myrn-engine-src-retrieve-rank-rs]]
