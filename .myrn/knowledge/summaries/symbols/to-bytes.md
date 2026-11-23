---
id: to-bytes
type: summary
status: unverified
title: to_bytes
source: myrn summarize
about: sym:crates/myrn-embed/src/lib.rs#to_bytes
anchors:
- sym:crates/myrn-embed/src/lib.rs#to_bytes@6e4a5c846bcf94a6
map_version: '2'
---
# to_bytes

Serialises a slice of f32 values into a byte vector, writing each value as 4 little-endian bytes in order.

## Behaviour

- [0.5] -> 4 bytes in little-endian f32 layout
- 3 values -> 12 bytes
- Empty slice -> empty Vec

## Terms

serialize vector, encode floats, f32 bytes, little endian, blob encoding, vector to blob

## Connections

- **Location:** `crates/myrn-embed/src/lib.rs:45-47`
- **Signature:** `pub fn to_bytes(v: &[f32]) -> Vec<u8>`
- **In:** [[file-crates-myrn-embed-src-lib-rs]]
- **Called by:** [[engine-update-vectors]]
- **Tests:** `bytes_round_trip`
