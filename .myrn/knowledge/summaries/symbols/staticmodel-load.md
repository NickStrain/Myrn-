---
id: staticmodel-load
type: summary
status: unverified
title: StaticModel::load
source: myrn summarize
about: sym:crates/myrn-embed/src/model.rs#model::StaticModel::load
anchors:
- sym:crates/myrn-embed/src/model.rs#model::StaticModel::load@5603148f36775184
map_version: '2'
---
# StaticModel::load

Reads tokenizer.json and model.safetensors from a directory and builds a StaticModel, reading the embeddings tensor from either 'embeddings' or 'embedding.weight'. Fails if the tokenizer or model file is missing, the tensor is absent, the tensor is not 2-D, or its dtype is unsupported.

## Behaviour

- dir with tokenizer.json and model.safetensors holding a [vocab, dim] tensor -> model with dim = shape[1]
- embeddings tensor only under 'embedding.weight' -> used as the fallback name
- 1-D embeddings tensor -> error 'expected [vocab, dim]'

## Terms

read model from disk, load model files, safetensors, tokenizer file, embedding tensor, model directory, weight dtype

## Connections

- **Location:** `crates/myrn-embed/src/model.rs:24-50`
- **Signature:** `pub fn load(dir: &Path) -> Result<StaticModel>`
- **In:** [[file-crates-myrn-embed-src-model-rs]]
- **Calls:** [[model-to-f32]]
- **Called by:** [[client-local]], [[helper-serve]], [[model-download]]
