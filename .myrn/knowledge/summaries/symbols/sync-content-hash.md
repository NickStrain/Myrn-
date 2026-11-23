---
id: sync-content-hash
type: summary
status: unverified
title: sync::content_hash
source: myrn summarize
about: sym:crates/myrn-store/src/sync.rs#sync::content_hash
anchors:
- sym:crates/myrn-store/src/sync.rs#sync::content_hash@3eb6ba6e8a724e12
map_version: '2'
---
# sync::content_hash

Returns a 16-character hex prefix of the blake3 digest of the given bytes.

## Behaviour

- same bytes always give the same string
- empty input gives the hash of zero bytes, still 16 characters
- output is lowercase hex, length 16

## Terms

file hash, blake3, digest, checksum, content fingerprint

## Connections

- **Location:** `crates/myrn-store/src/sync.rs:118-120`
- **Signature:** `pub fn content_hash(bytes: &[u8]) -> String`
- **In:** [[file-crates-myrn-store-src-sync-rs]]
- **Called by:** [[sync-body-hash]], [[sync-sync]]
