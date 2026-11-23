---
id: file-crates-myrn-engine-src-embeddings-rs
type: summary
status: unverified
title: crates/myrn-engine/src/embeddings.rs
source: myrn summarize
about: file:crates/myrn-engine/src/embeddings.rs
anchors:
- file:crates/myrn-engine/src/embeddings.rs@8c69d9255f2d7861
---
# crates/myrn-engine/src/embeddings.rs

Semantic search layer, enabled per project with 'myrn embeddings enable'. It builds text for each symbol, file and learned record, embeds it through myrn_embed into the vectors table, returns similarity-based seeds for a task, and provides enable, disable and status controls. A feature-gated download shim sits at the bottom.

## Connections

- **Location:** `crates/myrn-engine/src/embeddings.rs`
- **In:** [[dir-crates-myrn-engine-src]]
- **Defines:** [[embeddings-vectorreport]], [[embeddings-embeddingsstatus]], [[engine-data-home]], [[engine-embeddings-on]], [[engine-helper-exe]], [[engine-embed]], [[engine-update-vectors]], [[engine-vector-seeds]], [[engine-enable-embeddings]], [[engine-disable-embeddings]], [[engine-embeddings-status]], [[embeddings-download]], [[embeddings-download-embeddings]]
- **Imports:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-engine-src-retrieve-seed-rs]], [[file-crates-myrn-store-src-fts-rs]]
- **Imported by:** [[file-crates-myrn-engine-src-lib-rs]]
