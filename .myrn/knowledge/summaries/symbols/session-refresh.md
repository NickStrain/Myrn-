---
id: session-refresh
type: summary
status: unverified
title: session::refresh
source: myrn summarize
about: sym:tests/fixtures/repos/gate0/py/app/session.py#tests.fixtures.repos.gate0.py.app.session.refresh
anchors:
- sym:tests/fixtures/repos/gate0/py/app/session.py#tests.fixtures.repos.gate0.py.app.session.refresh@892f9c6ee669294e
map_version: '2'
---
# session::refresh

Looks up the token in the store and, if found, returns a new Session for it when that session is fresh; otherwise returns None.

## Behaviour

- store.lookup(token) returns None -> returns None and no Session is created
- store.lookup(token) returns a value -> returns a new Session built from that value
- The new Session is checked with is_fresh right after creation, so None is returned only if now() advances 10 or more units between the two calls

## Terms

reissue, renew, renewal, token exchange, new session, reissue session, token lookup, session renewal

## Connections

- **Location:** `tests/fixtures/repos/gate0/py/app/session.py:13-19`
- **Signature:** `def refresh(store: TokenStore, token)`
- **In:** [[file-tests-fixtures-repos-gate0-py-app-session-py]]
- **Calls:** [[session-session]], [[session-is-fresh]], [[tokenstore-lookup]]
- **Tests:** `test_refresh_known_token`
