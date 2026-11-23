---
id: tokenstore-lookup-store
type: summary
status: unverified
title: TokenStore::lookup
source: myrn summarize
about: sym:tests/fixtures/repos/gate0/src/store.rs#store::TokenStore::lookup
anchors:
- sym:tests/fixtures/repos/gate0/src/store.rs#store::TokenStore::lookup@06ca448fc00e7dcd
map_version: '2'
---
# TokenStore::lookup

Searches the stored tokens for an exact string match and returns a cloned copy of the first match, or None if absent.

## Behaviour

- "abc" stored, lookup("abc") -> Some("abc").
- Token not stored -> None.
- Empty store -> None. Comparison is exact and case-sensitive; if duplicates exist, the earliest-inserted one is returned.

## Terms

find token, search token, token check, validate token, token exists, match token

## Connections

- **Location:** `tests/fixtures/repos/gate0/src/store.rs:12-14`
- **Signature:** `pub fn lookup(&self, token: &str) -> Option<String>`
- **In:** [[file-tests-fixtures-repos-gate0-src-store-rs]]
- **Called by:** [[session-refresh-session]]
