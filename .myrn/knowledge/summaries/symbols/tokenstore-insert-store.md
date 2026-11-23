---
id: tokenstore-insert-store
type: summary
status: unverified
title: TokenStore::insert
source: myrn summarize
about: sym:tests/fixtures/repos/gate0/src/store.rs#store::TokenStore::insert
anchors:
- sym:tests/fixtures/repos/gate0/src/store.rs#store::TokenStore::insert@5c4f94699d47d504
map_version: '2'
---
# TokenStore::insert

Appends a copy of the given token string to the end of the stored list, mutating the store.

## Behaviour

- insert("abc") -> tokens now ends with "abc"; the input is copied with to_string.
- Inserting the same token twice stores it twice.
- Empty string is accepted and stored as-is.

## Terms

add token, store token, issue token, record token, push token, save token

## Connections

- **Location:** `tests/fixtures/repos/gate0/src/store.rs:16-18`
- **Signature:** `pub fn insert(&mut self, token: &str)`
- **In:** [[file-tests-fixtures-repos-gate0-src-store-rs]]
- **Tests:** `refreshes_known_token`
