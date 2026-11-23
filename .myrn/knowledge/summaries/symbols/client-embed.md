---
id: client-embed
type: summary
status: unverified
title: client::embed
source: myrn summarize
about: sym:crates/myrn-embed/src/client.rs#client::embed
anchors:
- sym:crates/myrn-embed/src/client.rs#client::embed@fc9567f589c833ec
map_version: '2'
---
# client::embed

Returns embedding vectors for texts, trying the running helper, then starting the helper and retrying, then loading the model locally.

## Behaviour

- empty texts -> Some(empty vec) with no work done
- model not installed or not supported -> None before any helper call
- helper_exe None or helper start fails -> skips to local model load

## Terms

vectorize, text to vector, embedding, fallback chain, helper or local, model not installed

## Connections

- **Location:** `crates/myrn-embed/src/client.rs:37-59`
- **Signature:** `pub fn embed( data_home: &Path, texts: &[String], helper_exe: Option<&Path>, ) -> Option<Vec<Vec<f32>>>`
- **In:** [[file-crates-myrn-embed-src-client-rs]]
- **Calls:** [[client-request]], [[client-start]], [[client-local-client]], [[supported]], [[installed]]
- **Called by:** [[engine-embed]]
- **Tests:** `without_embeddings_nothing_changes`
