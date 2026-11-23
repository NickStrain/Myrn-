---
id: store-tokenstore-store
type: summary
status: unverified
title: store::TokenStore
source: myrn summarize
about: sym:tests/fixtures/repos/gate0/src/store.rs#store::TokenStore
anchors:
- sym:tests/fixtures/repos/gate0/src/store.rs#store::TokenStore@6aa3b39e7d7a7477
map_version: '2'
---
# store::TokenStore

Holds the issued tokens in a private Vec<String> field. Tokens are kept in insertion order with no deduplication or expiry.

## Behaviour

- Constructed only through TokenStore::new, which starts with zero tokens.
- Duplicate tokens are stored again on each insert; no uniqueness check.
- Fields are private, so callers must use new, lookup and insert.

## Terms

token store, issued tokens, token list, token registry, in-memory storage

## Connections

- **Location:** `tests/fixtures/repos/gate0/src/store.rs:2-4`
- **Signature:** `pub struct TokenStore`
- **In:** [[file-tests-fixtures-repos-gate0-src-store-rs]]
