---
id: tokenstore-new
type: summary
status: unverified
title: TokenStore::new
source: myrn summarize
about: sym:tests/fixtures/repos/gate0/src/store.rs#store::TokenStore::new
anchors:
- sym:tests/fixtures/repos/gate0/src/store.rs#store::TokenStore::new@287aa2006af8fde4
map_version: '2'
---
# TokenStore::new

Returns an empty TokenStore with no tokens.

## Behaviour

- No arguments -> TokenStore with an empty tokens vector.
- Each call creates a new independent store.
- Takes no input and has no side effects.

## Terms

constructor, create store, initialise, empty store, default store

## Connections

- **Location:** `tests/fixtures/repos/gate0/src/store.rs:7-9`
- **Signature:** `pub fn new() -> TokenStore`
- **In:** [[file-tests-fixtures-repos-gate0-src-store-rs]]
- **Tests:** `refresh_unknown_token`, `refreshes_known_token`
