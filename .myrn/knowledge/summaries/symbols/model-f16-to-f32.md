---
id: model-f16-to-f32
type: summary
status: unverified
title: model::f16_to_f32
source: myrn summarize
about: sym:crates/myrn-embed/src/model.rs#model::f16_to_f32
anchors:
- sym:crates/myrn-embed/src/model.rs#model::f16_to_f32@7dce54e29f43c157
map_version: '2'
---
# model::f16_to_f32

Decodes one IEEE 754 half-precision value from its u16 bit pattern into an f32, handling zero, subnormals, infinities and NaN.

## Behaviour

- 0x3c00 -> 1.0; 0xc000 -> -2.0; 0x0000 -> 0.0
- 0x0001 (smallest subnormal) -> about 5.96e-8, normalised by shifting the mantissa
- 0x7c00 -> +infinity; 0x8000 -> -0.0 (sign bit kept for zero)

## Terms

half float, fp16 decode, subnormal, denormal, float16, sign bit, infinity, NaN

## Connections

- **Location:** `crates/myrn-embed/src/model.rs:108-128`
- **Signature:** `fn f16_to_f32(h: u16) -> f32`
- **In:** [[file-crates-myrn-embed-src-model-rs]]
- **Called by:** [[model-to-f32]]
- **Tests:** `half_floats`
