---
id: model-staticmodel
type: summary
status: unverified
title: model::StaticModel
source: myrn summarize
about: sym:crates/myrn-embed/src/model.rs#model::StaticModel
anchors:
- sym:crates/myrn-embed/src/model.rs#model::StaticModel@b80669888767c3bd
map_version: '2'
---
# model::StaticModel

Holds the tokenizer, the flattened embedding table, optional per-token weights, and the vocab size and dimension of a loaded static embedding model.

## Behaviour

- table is row-major [vocab, dim]; row id is at id * dim
- weights is None unless the file has a 'weights' tensor; a missing weight counts as 1.0
- dim is public; vocab is private and taken from the first axis of the embeddings tensor

## Terms

static embeddings, model2vec, embedding table, token vectors, vocabulary size, tokenizer, per-token weight

## Connections

- **Location:** `crates/myrn-embed/src/model.rs:13-21`
- **Signature:** `pub struct StaticModel`
- **In:** [[file-crates-myrn-embed-src-model-rs]]
