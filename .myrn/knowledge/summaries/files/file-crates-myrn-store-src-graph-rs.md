---
id: file-crates-myrn-store-src-graph-rs
type: summary
status: unverified
title: crates/myrn-store/src/graph.rs
source: myrn summarize
about: file:crates/myrn-store/src/graph.rs
anchors:
- file:crates/myrn-store/src/graph.rs@51be6cb3b3df2ff5
---
# crates/myrn-store/src/graph.rs

SQLite access layer for graph nodes and edges, built on rusqlite. It maps rows to Node, writes nodes and edges, looks them up, traverses neighbours and incident edges, and runs counts. The tests module holds a sym helper and a neighbours test.

## Connections

- **Location:** `crates/myrn-store/src/graph.rs`
- **In:** [[dir-crates-myrn-store-src]]
- **Defines:** [[graph-node]], [[graph-edge]], [[node-from-row]], [[node-location]], [[node-display-name]], [[graph-upsert-node]], [[graph-insert-edge]], [[graph-node-graph]], [[graph-nodes-where]], [[graph-dir]], [[graph-neighbors]], [[graph-edges-of]], [[graph-count]], [[tests-sym-graph]]
- **Imports:** [[file-crates-myrn-store-src-lib-rs]]
- **Imported by:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-engine-src-capture-episode-rs]], [[file-crates-myrn-engine-src-distill-checks-rs]], [[file-crates-myrn-engine-src-distill-record-rs]], [[file-crates-myrn-engine-src-indexer-git-rs]], [[file-crates-myrn-engine-src-indexer-mod-rs]], [[file-crates-myrn-engine-src-knowledge-rs]], [[file-crates-myrn-engine-src-pack-cards-rs]], [[file-crates-myrn-engine-src-pack-render-rs]], [[file-crates-myrn-engine-src-retrieve-rank-rs]], [[file-crates-myrn-engine-src-retrieve-recall-rs]], [[file-crates-myrn-engine-src-retrieve-seed-rs]], [[file-crates-myrn-engine-src-retrieve-walks-rs]], [[file-crates-myrn-engine-src-review-rs]], [[file-crates-myrn-engine-src-sharing-rs]], [[file-crates-myrn-engine-src-summarize-rs]], [[file-crates-myrn-engine-src-validity-anchors-rs]], [[file-crates-myrn-engine-src-validity-states-rs]], [[file-crates-myrn-engine-src-viz-rs]], [[file-crates-myrn-engine-tests-notes-rs]], [[file-crates-myrn-engine-tests-records-rs]], [[file-crates-myrn-store-src-lib-rs]], [[file-crates-myrn-store-src-sync-rs]]
