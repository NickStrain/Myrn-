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

Graph traversal helpers over the myrn store graph, used for impact analysis: listing a type's direct members, finding tests that call a symbol, a bounded breadth-first walk of incoming calls edges, and a count of non-test callers. Main parts are members, direct_tests, the Reach result struct, callers, and in_degree.

## Connections

- **Location:** `crates/myrn-engine/src/retrieve/walks.rs`
- **In:** [[dir-crates-myrn-engine-src-retrieve]]
- **Defines:** [[walks-members]], [[walks-direct-tests]], [[walks-reach]], [[walks-callers]], [[walks-in-degree]]
- **Imports:** [[file-crates-myrn-store-src-graph-rs]]
- **Imported by:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-engine-src-retrieve-mod-rs]], [[file-crates-myrn-engine-src-retrieve-rank-rs]]
