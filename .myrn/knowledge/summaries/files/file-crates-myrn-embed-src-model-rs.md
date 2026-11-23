---
id: file-crates-myrn-embed-src-model-rs
type: summary
status: unverified
title: crates/myrn-embed/src/model.rs
source: myrn summarize
about: file:crates/myrn-embed/src/model.rs
anchors:
- file:crates/myrn-embed/src/model.rs@159692a7d6476ba9
---
# crates/myrn-embed/src/model.rs

Static (Model2Vec) embedding model. It loads a tokenizer and a safetensors embedding table from a directory, encodes text as normalised sums of token rows, converts f16, bf16 and f32 weights to f32, and downloads the model files from Hugging Face. The main parts are the StaticModel struct with its load and encode methods, the to_f32 and f16_to_f32 dtype helpers, and download.

## Connections

- **Location:** `crates/myrn-embed/src/model.rs`
- **In:** [[dir-crates-myrn-embed-src]]
- **Defines:** [[model-staticmodel]], [[staticmodel-load]], [[staticmodel-encode]], [[model-to-f32]], [[model-f16-to-f32]], [[model-download]]
- **Imports:** [[file-crates-myrn-embed-src-lib-rs]]
- **Imported by:** [[file-crates-myrn-embed-src-lib-rs]]
