---
id: embeddings-download
type: summary
status: unverified
title: embeddings::download
source: myrn summarize
about: sym:crates/myrn-engine/src/embeddings.rs#embeddings::download
anchors:
- sym:crates/myrn-engine/src/embeddings.rs#embeddings::download@4341ee8725733cf6
map_version: '2'
---
# embeddings::download

Downloads the model into model_dir(home) via myrn_embed::model::download and forwards progress messages to the caller. Compiled only when the 'embeddings' feature is on.

## Behaviour

- Target directory is model_dir(home)
- Errors from model::download propagate unchanged
- The progress callback is passed through without changes

## Terms

model fetch, model install, weights download, progress callback, fetch model

## Connections

- **Location:** `crates/myrn-engine/src/embeddings.rs:248-250`
- **Signature:** `fn download(home: &std::path::Path, progress: &dyn Fn(&str)) -> Result<()>`
- **In:** [[file-crates-myrn-engine-src-embeddings-rs]]
- **Calls:** [[model-dir]], [[model-download]]
