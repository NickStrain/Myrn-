---
id: tokenstore-lookup
type: summary
status: unverified
title: TokenStore::lookup
source: myrn summarize
about: sym:tests/fixtures/repos/gate0/py/app/store.py#tests.fixtures.repos.gate0.py.app.store.TokenStore.lookup
anchors:
- sym:tests/fixtures/repos/gate0/py/app/store.py#tests.fixtures.repos.gate0.py.app.store.TokenStore.lookup@294ccef0a686855c
---
# TokenStore::lookup

Takes a token and returns it unchanged if present in `self.tokens`, otherwise returns None; linear-time membership check.

## Connections

- **Location:** `tests/fixtures/repos/gate0/py/app/store.py:7-8`
- **Signature:** `def lookup(self, token)`
- **In:** [[file-tests-fixtures-repos-gate0-py-app-store-py]]
- **Called by:** [[session-refresh]]
