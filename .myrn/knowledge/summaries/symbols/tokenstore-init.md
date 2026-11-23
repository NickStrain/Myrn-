---
id: tokenstore-init
type: summary
status: unverified
title: TokenStore::__init__
source: myrn summarize
about: sym:tests/fixtures/repos/gate0/py/app/store.py#tests.fixtures.repos.gate0.py.app.store.TokenStore.__init__
anchors:
- sym:tests/fixtures/repos/gate0/py/app/store.py#tests.fixtures.repos.gate0.py.app.store.TokenStore.__init__@08b821d5c881bf6c
map_version: '2'
---
# TokenStore::__init__

Creates the store with an empty list at self.tokens.

## Behaviour

- TokenStore() -> self.tokens is []
- each instance gets its own new list, not shared state

## Terms

constructor, initialise, empty list, setup

## Connections

- **Location:** `tests/fixtures/repos/gate0/py/app/store.py:4-5`
- **Signature:** `def __init__(self)`
- **In:** [[file-tests-fixtures-repos-gate0-py-app-store-py]]
