---
id: tokenstore-lookup-4e6a
type: summary
status: unverified
title: TokenStore::lookup
source: myrn summarize
about: sym:tests/fixtures/repos/gate0/web/src/store.ts#TokenStore.lookup
anchors:
- sym:tests/fixtures/repos/gate0/web/src/store.ts#TokenStore.lookup@61453401771def1a
map_version: '2'
---
# TokenStore::lookup

Returns the stored token string that strictly equals the argument, or undefined if none matches.

## Behaviour

- lookup('x') with 'x' stored -> 'x'
- lookup('X') when only 'x' stored -> undefined (case-sensitive)
- empty store -> undefined

## Terms

find token, check token, validate token, token exists, token match

## Connections

- **Location:** `tests/fixtures/repos/gate0/web/src/store.ts:5-7`
- **Signature:** `lookup(token: string): string | undefined`
- **In:** [[file-tests-fixtures-repos-gate0-web-src-store-ts]]
- **Called by:** [[refresh]]
