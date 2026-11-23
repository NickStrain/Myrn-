---
id: model-to-f32
type: summary
status: unverified
title: model::to_f32
source: myrn summarize
about: sym:crates/myrn-embed/src/model.rs#model::to_f32
anchors:
- sym:crates/myrn-embed/src/model.rs#model::to_f32@3e35a8fdd87414df
map_version: '2'
---
# model::to_f32

Converts raw little-endian tensor bytes of dtype F32, F16 or BF16 into a Vec<f32>, and errors on any other dtype.

## Behaviour

- F32 bytes [0,0,128,63] -> [1.0]
- BF16 bytes [0x80,0x3f] -> [1.0] (bits shifted into the high 16 bits)
- integer or other dtype -> error 'unsupported weight type'; a trailing partial chunk is silently dropped

## Terms

dtype conversion, bytes to floats, half precision, bfloat16, fp16, tensor decoding, little endian

## Connections

- **Location:** `crates/myrn-embed/src/model.rs:90-106`
- **Signature:** `fn to_f32(dtype: Dtype, data: &[u8]) -> Result<Vec<f32>>`
- **In:** [[file-crates-myrn-embed-src-model-rs]]
- **Calls:** [[model-f16-to-f32]]
- **Called by:** [[staticmodel-load]]
