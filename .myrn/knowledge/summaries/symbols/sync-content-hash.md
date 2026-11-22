---
id: sync-content-hash
type: summary
status: unverified
title: sync::content_hash
source: myrn summarize
about: sym:crates/myrn-store/src/sync.rs#sync::content_hash
anchors:
- sym:crates/myrn-store/src/sync.rs#sync::content_hash@3eb6ba6e8a724e12
---
# sync::content_hash

Returns the first 16 hex characters of the BLAKE3 hash of the given bytes, used to detect content changes.

## Connections

- **Location:** `crates/myrn-store/src/sync.rs:118-120`
- **Signature:** `pub fn content_hash(bytes: &[u8]) -> String`
- **In:** [[file-crates-myrn-store-src-sync-rs]]
- **Called by:** [[sync-sync]]
