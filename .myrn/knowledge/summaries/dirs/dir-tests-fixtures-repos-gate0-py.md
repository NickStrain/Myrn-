---
id: dir-tests-fixtures-repos-gate0-py
type: summary
status: unverified
title: tests/fixtures/repos/gate0/py/
source: myrn summarize
about: module:tests/fixtures/repos/gate0/py
anchors:
- module:tests/fixtures/repos/gate0/py@7be45d39e25d4a65
---
# tests/fixtures/repos/gate0/py/

Fixture Python package and test for the gate0 test repo's session-token refresh path. app/ holds a session-token model (session.py), an in-memory token registry (store.py), and an empty __init__.py; session.refresh() looks a token up in a TokenStore and issues a new Session, with timestamps from now() in store.py. tests/ has one test, test_refresh_known_token, which seeds a TokenStore with "abc" and asserts refresh(store, "abc") returns non-None.

## Connections

- **In:** [[dir-tests-fixtures-repos-gate0]]
- **Directories:** [[dir-tests-fixtures-repos-gate0-py-app]], [[dir-tests-fixtures-repos-gate0-py-tests]]
