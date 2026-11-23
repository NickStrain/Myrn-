---
id: tokenstore-insert
type: summary
status: unverified
title: TokenStore::insert
source: myrn summarize
about: sym:tests/fixtures/repos/gate0/py/app/store.py#tests.fixtures.repos.gate0.py.app.store.TokenStore.insert
anchors:
- sym:tests/fixtures/repos/gate0/py/app/store.py#tests.fixtures.repos.gate0.py.app.store.TokenStore.insert@193e38c5ecdc6366
map_version: '2'
---
# TokenStore::insert

Appends the token to self.tokens. Does not check for duplicates and returns nothing.

## Behaviour

- insert('x') twice -> list contains 'x' twice
- insert returns None
- insert mutates the store in place; no copy is made

## Terms

add token, append, store token, record token, issue token, save token

## Connections

- **Location:** `tests/fixtures/repos/gate0/py/app/store.py:10-11`
- **Signature:** `def insert(self, token)`
- **In:** [[file-tests-fixtures-repos-gate0-py-app-store-py]]
- **Tests:** `test_refresh_known_token`
