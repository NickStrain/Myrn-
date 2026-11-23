---
id: indexer-content-hash
type: summary
status: unverified
title: indexer::content_hash
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/mod.rs#indexer::content_hash
anchors:
- sym:crates/myrn-engine/src/indexer/mod.rs#indexer::content_hash@3eb6ba6e8a724e12
map_version: '2'
---
# indexer::content_hash

Returns the first 16 lowercase hex characters of the BLAKE3 hash of the bytes.

## Behaviour

- same bytes -> same string
- empty slice -> still 16 hex characters
- changed bytes -> normally a different string, since only 64 bits are kept

## Terms

digest, checksum, fingerprint, blake3, file hash, truncated hash

## Connections

- **Location:** `crates/myrn-engine/src/indexer/mod.rs:48-50`
- **Signature:** `pub fn content_hash(bytes: &[u8]) -> String`
- **In:** [[file-crates-myrn-engine-src-indexer-mod-rs]]
- **Called by:** [[api-locate]], [[indexer-refresh]]
