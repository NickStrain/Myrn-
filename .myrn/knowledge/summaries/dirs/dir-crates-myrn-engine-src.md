---
id: dir-crates-myrn-engine-src
type: summary
status: unverified
title: crates/myrn-engine/src/
source: myrn summarize
about: module:crates/myrn-engine/src
anchors:
- module:crates/myrn-engine/src@615d1a60b102a7e3
---
# crates/myrn-engine/src/

Core engine of the Myrn tool. indexer/ builds a SQLite code graph from the working tree and git history, capture/ and distill/ turn agent sessions into validated knowledge records, and retrieve/, pack/ and validity/ use that graph and those records to find, pack and check context. Adapters reach all of it only through api.rs, the sole code that touches the stores, while config.rs, learn.rs, embeddings.rs, summarize.rs and review.rs add settings, ranking feedback, semantic seeds, summaries and hand review.

## Connections

- **In:** [[dir-crates-myrn-engine]]
- **Directories:** [[dir-crates-myrn-engine-src-capture]], [[dir-crates-myrn-engine-src-distill]], [[dir-crates-myrn-engine-src-indexer]], [[dir-crates-myrn-engine-src-pack]], [[dir-crates-myrn-engine-src-retrieve]], [[dir-crates-myrn-engine-src-validity]]
- **Files:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-engine-src-config-rs]], [[file-crates-myrn-engine-src-embeddings-rs]], [[file-crates-myrn-engine-src-knowledge-rs]], [[file-crates-myrn-engine-src-learn-rs]], [[file-crates-myrn-engine-src-lib-rs]], [[file-crates-myrn-engine-src-review-rs]], [[file-crates-myrn-engine-src-sharing-rs]], [[file-crates-myrn-engine-src-summarize-rs]], [[file-crates-myrn-engine-src-viz-rs]]
