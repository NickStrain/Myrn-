---
id: dir-crates-myrn-engine-src-retrieve
type: summary
status: unverified
title: crates/myrn-engine/src/retrieve/
source: myrn summarize
about: module:crates/myrn-engine/src/retrieve
anchors:
- module:crates/myrn-engine/src/retrieve@454b21a5c23fd5ce
---
# crates/myrn-engine/src/retrieve/

Retrieval subsystem for the myrn engine, built as seeded, typed graph traversal rather than plain similarity search. mod.rs only declares the submodules: seed (picks start nodes from symbols, paths, path:line locations, or full-text matches), walks (bounded BFS over incoming calls edges plus direct-member and test lookups), rank (one-hop expansion and scoring with a MIN_SCORE gate of 3.0), and recall (bug lookup by normalized error signature, rendered into a token-budgeted block).

## Connections

- **In:** [[dir-crates-myrn-engine-src]]
- **Files:** [[file-crates-myrn-engine-src-retrieve-mod-rs]], [[file-crates-myrn-engine-src-retrieve-rank-rs]], [[file-crates-myrn-engine-src-retrieve-recall-rs]], [[file-crates-myrn-engine-src-retrieve-seed-rs]], [[file-crates-myrn-engine-src-retrieve-walks-rs]]
