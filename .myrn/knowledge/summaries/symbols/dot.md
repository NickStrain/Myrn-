---
id: dot
type: summary
status: unverified
title: dot
source: myrn summarize
about: sym:crates/myrn-embed/src/lib.rs#dot
anchors:
- sym:crates/myrn-embed/src/lib.rs#dot@ac7ffa30f1a861d4
map_version: '2'
---
# dot

Computes the sum of element-wise products of two slices, which equals cosine similarity when both vectors are L2-normalised.

## Behaviour

- [0.6, 0.8] with [0.6, 0.8] -> 1.0
- Slices of different lengths: iteration stops at the shorter one, so extra elements are ignored
- Empty slices -> 0.0

## Terms

cosine similarity, dot product, inner product, similarity score, normalised vectors, vector comparison

## Connections

- **Location:** `crates/myrn-embed/src/lib.rs:41-43`
- **Signature:** `pub fn dot(a: &[f32], b: &[f32]) -> f32`
- **In:** [[file-crates-myrn-embed-src-lib-rs]]
- **Called by:** [[main-simcheck]], [[engine-vector-seeds]]
- **Tests:** `bytes_round_trip`
