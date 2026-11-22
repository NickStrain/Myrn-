---
id: session-refresh
type: summary
status: unverified
title: session::refresh
source: myrn summarize
about: sym:tests/fixtures/repos/gate0/py/app/session.py#tests.fixtures.repos.gate0.py.app.session.refresh
anchors:
- sym:tests/fixtures/repos/gate0/py/app/session.py#tests.fixtures.repos.gate0.py.app.session.refresh@892f9c6ee669294e
---
# session::refresh

Takes a TokenStore and token; returns None if store.lookup(token) finds nothing, otherwise builds a Session from the found value and returns it only if fresh, else None. Does not write to the store.

## Connections

- **Location:** `tests/fixtures/repos/gate0/py/app/session.py:13-19`
- **Signature:** `def refresh(store: TokenStore, token)`
- **In:** [[file-tests-fixtures-repos-gate0-py-app-session-py]]
- **Calls:** [[session-session]], [[session-is-fresh]], [[tokenstore-lookup]]
- **Tests:** `test_refresh_known_token`
