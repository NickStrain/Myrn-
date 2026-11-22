---
id: tokenstore-lookup-4e6a
type: summary
status: unverified
title: TokenStore::lookup
source: myrn summarize
about: sym:tests/fixtures/repos/gate0/web/src/store.ts#TokenStore.lookup
anchors:
- sym:tests/fixtures/repos/gate0/web/src/store.ts#TokenStore.lookup@61453401771def1a
---
# TokenStore::lookup

Takes a token string and returns the matching stored string via linear find with strict equality, or undefined if absent; does not mutate state.

## Connections

- **Location:** `tests/fixtures/repos/gate0/web/src/store.ts:5-7`
- **Signature:** `lookup(token: string): string | undefined`
- **In:** [[file-tests-fixtures-repos-gate0-web-src-store-ts]]
- **Called by:** [[refresh]]
