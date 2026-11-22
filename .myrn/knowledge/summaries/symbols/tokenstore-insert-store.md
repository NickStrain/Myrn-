---
id: tokenstore-insert-store
type: summary
status: unverified
title: TokenStore::insert
source: myrn summarize
about: sym:tests/fixtures/repos/gate0/src/store.rs#store::TokenStore::insert
anchors:
- sym:tests/fixtures/repos/gate0/src/store.rs#store::TokenStore::insert@5c4f94699d47d504
---
# TokenStore::insert

Takes &mut self and a &str token; appends an owned copy to the list without checking for duplicates. Returns nothing.

## Connections

- **Location:** `tests/fixtures/repos/gate0/src/store.rs:16-18`
- **Signature:** `pub fn insert(&mut self, token: &str)`
- **In:** [[file-tests-fixtures-repos-gate0-src-store-rs]]
- **Tests:** `refreshes_known_token`
