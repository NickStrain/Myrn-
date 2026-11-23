---
id: client-local
type: summary
status: unverified
title: client::local
source: myrn summarize
about: sym:crates/myrn-embed/src/client.rs#client::local
anchors:
- sym:crates/myrn-embed/src/client.rs#client::local@3a137fbd416dde6b
map_version: '2'
---
# client::local

Loads the static model from the model directory and encodes texts in this process when the model feature is compiled in.

## Behaviour

- model directory fails to load -> None
- encode returns an error -> None
- empty texts are passed straight to encode

## Terms

in-process model, load model, encode locally, static model, bulk embedding, no helper

## Connections

- **Location:** `crates/myrn-embed/src/client.rs:86-89`
- **Signature:** `pub fn local(data_home: &Path, texts: &[String]) -> Option<Vec<Vec<f32>>>`
- **In:** [[file-crates-myrn-embed-src-client-rs]]
- **Calls:** [[model-dir]], [[staticmodel-load]], [[staticmodel-encode]]
