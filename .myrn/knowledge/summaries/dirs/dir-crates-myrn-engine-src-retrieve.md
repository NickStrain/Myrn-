---
id: dir-crates-myrn-engine-src-retrieve
type: summary
status: unverified
title: crates/myrn-engine/src/retrieve/
source: myrn summarize
about: module:crates/myrn-engine/src/retrieve
anchors:
- module:crates/myrn-engine/src/retrieve@d064837d9887aa21
---
# crates/myrn-engine/src/retrieve/

Retrieval subsystem of the myrn engine: finds relevant code and known-bug records by seeding a graph from a task or error text, then walking typed edges (calls, tests, notes, bugs) rather than relying on plain similarity search. Flow is seed.rs (pick starting nodes) → walks.rs (graph traversal helpers) → rank.rs (one-hop expansion and ordering), with recall.rs providing known-bug lookup from error text; mod.rs only declares these four submodules.

## Connections

- **In:** [[dir-crates-myrn-engine-src]]
- **Files:** [[file-crates-myrn-engine-src-retrieve-mod-rs]], [[file-crates-myrn-engine-src-retrieve-rank-rs]], [[file-crates-myrn-engine-src-retrieve-recall-rs]], [[file-crates-myrn-engine-src-retrieve-seed-rs]], [[file-crates-myrn-engine-src-retrieve-walks-rs]]
