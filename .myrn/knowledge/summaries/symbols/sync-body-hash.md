---
id: sync-body-hash
type: summary
status: unverified
title: sync::body_hash
source: myrn summarize
about: sym:crates/myrn-store/src/sync.rs#sync::body_hash
anchors:
- sym:crates/myrn-store/src/sync.rs#sync::body_hash@846402c2e5db223f
map_version: '2'
---
# sync::body_hash

Hashes a record's body after collapsing all runs of whitespace into single spaces, so layout changes do not change the hash.

## Behaviour

- 'a b\nc' and 'a b c' give the same hash
- leading and trailing whitespace is ignored
- empty or whitespace-only body hashes the empty string

## Terms

normalized hash, whitespace-insensitive hash, record hash, verification hash, body fingerprint

## Connections

- **Location:** `crates/myrn-store/src/sync.rs:124-127`
- **Signature:** `pub fn body_hash(body: &str) -> String`
- **In:** [[file-crates-myrn-store-src-sync-rs]]
- **Calls:** [[sync-content-hash]]
- **Called by:** [[sync-link]], [[sync-set-local-verified]]
