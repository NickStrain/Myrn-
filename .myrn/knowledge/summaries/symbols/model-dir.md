---
id: model-dir
type: summary
status: unverified
title: model_dir
source: myrn summarize
about: sym:crates/myrn-embed/src/lib.rs#model_dir
anchors:
- sym:crates/myrn-embed/src/lib.rs#model_dir@0b28ec89d5b4de09
map_version: '2'
---
# model_dir

Returns the per-user model directory: `models/` joined under the given data home, plus the last `/` segment of MODEL_REPO.

## Behaviour

- data_home `/home/u/.local/share` -> `/home/u/.local/share/models/potion-base-8M`
- Only the final path segment of MODEL_REPO is used, so the org prefix `minishlab` is dropped
- Does not check that the directory exists

## Terms

model path, model location, data home, install directory, model folder, cache path

## Connections

- **Location:** `crates/myrn-embed/src/lib.rs:23-27`
- **Signature:** `pub fn model_dir(data_home: &Path) -> PathBuf`
- **In:** [[file-crates-myrn-embed-src-lib-rs]]
- **Called by:** [[real-main]], [[client-local]], [[helper-serve]], [[installed]], [[engine-enable-embeddings]], [[engine-embeddings-status]], [[embeddings-download]]
