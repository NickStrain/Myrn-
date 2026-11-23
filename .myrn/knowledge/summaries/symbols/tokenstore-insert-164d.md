---
id: tokenstore-insert-164d
type: summary
status: unverified
title: TokenStore::insert
source: myrn summarize
about: sym:tests/fixtures/repos/gate0/web/src/store.ts#TokenStore.insert
anchors:
- sym:tests/fixtures/repos/gate0/web/src/store.ts#TokenStore.insert@33cd82562dc80c55
map_version: '2'
---
# TokenStore::insert

Appends the given token to the internal array with no validation, returning nothing.

## Behaviour

- insert('a') -> array gains 'a' at the end
- insert('a') twice -> 'a' appears twice
- insert('') -> empty string is stored as-is

## Terms

add token, store token, push token, issue token, record token

## Connections

- **Location:** `tests/fixtures/repos/gate0/web/src/store.ts:9-11`
- **Signature:** `insert(token: string): void`
- **In:** [[file-tests-fixtures-repos-gate0-web-src-store-ts]]
- **Tests:** `returns a session for a known token`
