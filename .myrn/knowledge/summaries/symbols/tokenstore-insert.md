---
id: tokenstore-insert
type: summary
status: unverified
title: TokenStore::insert
source: myrn summarize
about: sym:tests/fixtures/repos/gate0/py/app/store.py#tests.fixtures.repos.gate0.py.app.store.TokenStore.insert
anchors:
- sym:tests/fixtures/repos/gate0/py/app/store.py#tests.fixtures.repos.gate0.py.app.store.TokenStore.insert@193e38c5ecdc6366
---
# TokenStore::insert

Appends the given token to `self.tokens` without checking for duplicates; returns None and mutates store state.

## Connections

- **Location:** `tests/fixtures/repos/gate0/py/app/store.py:10-11`
- **Signature:** `def insert(self, token)`
- **In:** [[file-tests-fixtures-repos-gate0-py-app-store-py]]
- **Tests:** `test_refresh_known_token`
