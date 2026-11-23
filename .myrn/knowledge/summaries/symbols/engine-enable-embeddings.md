---
id: engine-enable-embeddings
type: summary
status: unverified
title: Engine::enable_embeddings
source: myrn summarize
about: sym:crates/myrn-engine/src/embeddings.rs#embeddings::Engine::enable_embeddings
anchors:
- sym:crates/myrn-engine/src/embeddings.rs#embeddings::Engine::enable_embeddings@3dec9262f32923b1
map_version: '2'
---
# Engine::enable_embeddings

Downloads the model into data_home if it is missing (reporting progress), sets embeddings true in project state, runs update_vectors, and returns its report.

## Behaviour

- Build without embedding support -> error before any download; the message names the 'model' feature, while download() uses the 'embeddings' feature
- Model already installed -> no download, just state flip and embed
- Download fails -> error returned and state is not saved as enabled

## Terms

turn on, enable, install model, download model, first run setup, embed everything

## Connections

- **Location:** `crates/myrn-engine/src/embeddings.rs:200-220`
- **Signature:** `pub fn enable_embeddings(&mut self, progress: &dyn Fn(&str)) -> Result<VectorReport>`
- **In:** [[file-crates-myrn-engine-src-embeddings-rs]]
- **Calls:** [[model-dir]], [[supported]], [[installed]], [[engine-state]], [[engine-save-state]], [[engine-data-home]], [[engine-update-vectors]], [[embeddings-download-embeddings]]
- **Called by:** [[embeddings-enable]]
