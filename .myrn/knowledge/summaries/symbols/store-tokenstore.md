---
id: store-tokenstore
type: summary
status: unverified
title: store::TokenStore
source: myrn summarize
about: sym:tests/fixtures/repos/gate0/py/app/store.py#tests.fixtures.repos.gate0.py.app.store.TokenStore
anchors:
- sym:tests/fixtures/repos/gate0/py/app/store.py#tests.fixtures.repos.gate0.py.app.store.TokenStore@41decb11e95ca224
map_version: '2'
---
# store::TokenStore

In-memory store that holds issued tokens in a Python list, supporting membership lookup and append.

## Behaviour

- TokenStore() -> empty store, tokens == []
- insert(t) then lookup(t) -> returns t
- lookup(t) on a token never inserted -> None

## Terms

token store, token registry, issued tokens, in-memory storage, token list

## Connections

- **Location:** `tests/fixtures/repos/gate0/py/app/store.py:1-11`
- **Signature:** `class TokenStore`
- **In:** [[file-tests-fixtures-repos-gate0-py-app-store-py]]
- **Tests:** `test_refresh_known_token`
