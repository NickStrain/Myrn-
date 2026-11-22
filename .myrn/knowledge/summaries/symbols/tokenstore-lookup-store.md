---
id: tokenstore-lookup-store
type: summary
status: unverified
title: TokenStore::lookup
source: myrn summarize
about: sym:tests/fixtures/repos/gate0/src/store.rs#store::TokenStore::lookup
anchors:
- sym:tests/fixtures/repos/gate0/src/store.rs#store::TokenStore::lookup@06ca448fc00e7dcd
---
# TokenStore::lookup

Takes &self and a &str token; returns Some(owned String) of the first stored token that exactly matches, or None. Linear scan of the Vec.

## Connections

- **Location:** `tests/fixtures/repos/gate0/src/store.rs:12-14`
- **Signature:** `pub fn lookup(&self, token: &str) -> Option<String>`
- **In:** [[file-tests-fixtures-repos-gate0-src-store-rs]]
- **Called by:** [[session-refresh-session]]
