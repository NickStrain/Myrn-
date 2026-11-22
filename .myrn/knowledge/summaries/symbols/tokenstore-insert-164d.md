---
id: tokenstore-insert-164d
type: summary
status: unverified
title: TokenStore::insert
source: myrn summarize
about: sym:tests/fixtures/repos/gate0/web/src/store.ts#TokenStore.insert
anchors:
- sym:tests/fixtures/repos/gate0/web/src/store.ts#TokenStore.insert@33cd82562dc80c55
---
# TokenStore::insert

Appends the given token string to the internal list with no deduplication or validation, so inserting the same token twice stores two entries; returns void.

## Connections

- **Location:** `tests/fixtures/repos/gate0/web/src/store.ts:9-11`
- **Signature:** `insert(token: string): void`
- **In:** [[file-tests-fixtures-repos-gate0-web-src-store-ts]]
- **Tests:** `returns a session for a known token`
