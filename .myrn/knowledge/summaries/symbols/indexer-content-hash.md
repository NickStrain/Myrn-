---
id: indexer-content-hash
type: summary
status: unverified
title: indexer::content_hash
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/mod.rs#indexer::content_hash
anchors:
- sym:crates/myrn-engine/src/indexer/mod.rs#indexer::content_hash@3eb6ba6e8a724e12
---
# indexer::content_hash

Returns the first 16 hex characters of the blake3 hash of the given bytes. Used as the files.content_hash and body_hash values.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/mod.rs:48-50`
- **Signature:** `pub fn content_hash(bytes: &[u8]) -> String`
- **In:** [[file-crates-myrn-engine-src-indexer-mod-rs]]
- **Called by:** [[api-locate]], [[indexer-refresh]]
