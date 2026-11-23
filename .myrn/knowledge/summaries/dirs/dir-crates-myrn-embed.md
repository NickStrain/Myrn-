---
id: dir-crates-myrn-embed
type: summary
status: unverified
title: crates/myrn-embed/
source: myrn summarize
about: module:crates/myrn-embed
anchors:
- module:crates/myrn-embed@9609de4293d6d4cd
---
# crates/myrn-embed/

Optional Model2Vec static-embedding support for myrn, split into three parts: a client that requests embeddings, a helper process that loads the model once and serves them over a token-guarded localhost TCP port, and a model module that loads, encodes, and downloads the weights. lib.rs holds shared constants, path and availability helpers, similarity and f32 byte-encoding utilities, and gates the model module behind a feature flag; callers fall back to lexical search whenever the client returns None.

## Connections

- **In:** [[dir-crates]]
- **Directories:** [[dir-crates-myrn-embed-src]]
