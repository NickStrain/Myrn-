---
id: model-download
type: summary
status: unverified
title: model::download
source: myrn summarize
about: sym:crates/myrn-embed/src/model.rs#model::download
anchors:
- sym:crates/myrn-embed/src/model.rs#model::download@45e113bd7c5bdf12
map_version: '2'
---
# model::download

Creates the directory, fetches each MODEL_FILES entry from the Hugging Face repo over HTTPS, writes each file through a .part temp file and renames it into place, then loads the result to confirm it is usable. It overwrites existing files and makes network calls and filesystem writes.

## Behaviour

- a missing dir is created; progress is called with 'downloading <file>' once per file
- each body is read only up to 512 MiB, and a larger body is cut off without an error
- a failed load after download -> error 'the downloaded model does not load'; no checksum is verified

## Terms

fetch model, install model, huggingface, model download, http download, temp file, atomic rename, progress callback

## Connections

- **Location:** `crates/myrn-embed/src/model.rs:132-152`
- **Signature:** `pub fn download(dir: &Path, progress: &dyn Fn(&str)) -> Result<()>`
- **In:** [[file-crates-myrn-embed-src-model-rs]]
- **Calls:** [[staticmodel-load]]
- **Called by:** [[embeddings-download]]
