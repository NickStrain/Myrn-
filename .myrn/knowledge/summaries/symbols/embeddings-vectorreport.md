---
id: embeddings-vectorreport
type: summary
status: unverified
title: embeddings::VectorReport
source: myrn summarize
about: sym:crates/myrn-engine/src/embeddings.rs#embeddings::VectorReport
anchors:
- sym:crates/myrn-engine/src/embeddings.rs#embeddings::VectorReport@ab240a1786905c35
map_version: '2'
---
# embeddings::VectorReport

Counts from one update_vectors run: nodes embedded, stale vectors removed, nodes considered, and elapsed milliseconds.

## Behaviour

- Default -> all fields zero
- embedded counts only nodes whose stored hash differs from the current hash, or that have no vector yet
- total counts candidate rows selected from nodes, not every node in the database

## Terms

embedding stats, update result, counts, timing, removed stale vectors, run summary

## Connections

- **Location:** `crates/myrn-engine/src/embeddings.rs:21-26`
- **Signature:** `pub struct VectorReport`
- **In:** [[file-crates-myrn-engine-src-embeddings-rs]]
