---
id: tokenstore-lookup
type: summary
status: unverified
title: TokenStore::lookup
source: myrn summarize
about: sym:tests/fixtures/repos/gate0/py/app/store.py#tests.fixtures.repos.gate0.py.app.store.TokenStore.lookup
anchors:
- sym:tests/fixtures/repos/gate0/py/app/store.py#tests.fixtures.repos.gate0.py.app.store.TokenStore.lookup@294ccef0a686855c
map_version: '2'
---
# TokenStore::lookup

Returns the given token if it is present in self.tokens, otherwise None. Membership test is a linear list scan.

## Behaviour

- lookup('abc') when 'abc' was inserted -> 'abc'
- lookup('abc') on empty store -> None
- lookup of a falsy value such as '' returns '' only if '' was inserted, otherwise None

## Terms

find token, check token, token exists, validate token, membership check, search

## Connections

- **Location:** `tests/fixtures/repos/gate0/py/app/store.py:7-8`
- **Signature:** `def lookup(self, token)`
- **In:** [[file-tests-fixtures-repos-gate0-py-app-store-py]]
- **Called by:** [[session-refresh]]
