---
id: dir-crates-myrn-embed-src
type: summary
status: unverified
title: crates/myrn-embed/src/
source: myrn summarize
about: module:crates/myrn-embed/src
anchors:
- module:crates/myrn-embed/src@5f9e0e609c11583c
---
# crates/myrn-embed/src/

Optional Model2Vec static-embedding support for the myrn tool, split into a client that requests embeddings, a helper process that loads the model once and serves them over a token-guarded localhost TCP port, and a model module that loads, encodes, and downloads the Model2Vec weights. lib.rs holds the shared constants, path and availability helpers, similarity and f32 byte-encoding utilities, and gates the model module behind a feature flag; callers fall back to lexical search whenever the client returns None.

## Connections

- **In:** [[dir-crates-myrn-embed]]
- **Files:** [[file-crates-myrn-embed-src-client-rs]], [[file-crates-myrn-embed-src-helper-rs]], [[file-crates-myrn-embed-src-lib-rs]], [[file-crates-myrn-embed-src-model-rs]]
