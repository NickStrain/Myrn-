---
id: from-bytes
type: summary
status: unverified
title: from_bytes
source: myrn summarize
about: sym:crates/myrn-embed/src/lib.rs#from_bytes
anchors:
- sym:crates/myrn-embed/src/lib.rs#from_bytes@0771f821fdf03a1f
map_version: '2'
---
# from_bytes

Decodes a byte slice into f32 values, reading each complete 4-byte little-endian chunk; it is the inverse of to_bytes.

## Behaviour

- 12 bytes -> 3 f32 values
- Trailing 1 to 3 bytes that do not form a full chunk are silently ignored
- Empty slice -> empty Vec

## Terms

deserialize vector, decode floats, parse blob, f32 bytes, little endian, blob to vector

## Connections

- **Location:** `crates/myrn-embed/src/lib.rs:49-53`
- **Signature:** `pub fn from_bytes(b: &[u8]) -> Vec<f32>`
- **In:** [[file-crates-myrn-embed-src-lib-rs]]
- **Called by:** [[main-simcheck]], [[engine-vector-seeds]]
- **Tests:** `bytes_round_trip`
