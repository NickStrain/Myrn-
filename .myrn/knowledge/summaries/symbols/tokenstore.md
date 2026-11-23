---
id: tokenstore
type: summary
status: unverified
title: TokenStore
source: myrn summarize
about: sym:tests/fixtures/repos/gate0/web/src/store.ts#TokenStore
anchors:
- sym:tests/fixtures/repos/gate0/web/src/store.ts#TokenStore@a1555e7cf80590f9
map_version: '2'
---
# TokenStore

In-memory store holding issued tokens as a private string array, exposing lookup and insert.

## Behaviour

- insert('abc') then lookup('abc') -> 'abc'
- lookup on a token never inserted -> undefined
- duplicate inserts are kept; the array is not deduplicated

## Terms

token store, issued tokens, token list, token registry, session token

## Connections

- **Location:** `tests/fixtures/repos/gate0/web/src/store.ts:2-12`
- **Signature:** `class TokenStore`
- **In:** [[file-tests-fixtures-repos-gate0-web-src-store-ts]]
- **Tests:** `returns a session for a known token`
