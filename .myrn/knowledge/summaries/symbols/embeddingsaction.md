---
id: embeddingsaction
type: summary
status: unverified
title: EmbeddingsAction
source: myrn summarize
about: sym:crates/myrn/src/main.rs#EmbeddingsAction
anchors:
- sym:crates/myrn/src/main.rs#EmbeddingsAction@f510b67473b03ffc
map_version: '2'
---
# EmbeddingsAction

Subcommand enum for `myrn embeddings` controlling the optional semantic search: enable, disable, or status.

## Behaviour

- `enable` downloads the roughly 30 MB model once per machine and turns semantic search on for this store
- `disable` turns it off here and deletes the vectors but keeps the downloaded model
- `status` reports whether it is on, plus the model, vectors and helper process

## Terms

semantic search toggle, embedding model download, vector store, helper process, on off switch

## Connections

- **Location:** `crates/myrn/src/main.rs:209-216`
- **Signature:** `enum EmbeddingsAction`
- **In:** [[file-crates-myrn-src-main-rs]]
