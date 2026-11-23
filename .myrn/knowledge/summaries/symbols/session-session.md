---
id: session-session
type: summary
status: unverified
title: session::Session
source: myrn summarize
about: sym:tests/fixtures/repos/gate0/py/app/session.py#tests.fixtures.repos.gate0.py.app.session.Session
anchors:
- sym:tests/fixtures/repos/gate0/py/app/session.py#tests.fixtures.repos.gate0.py.app.session.Session@0c0719a75c6b0762
map_version: '2'
---
# session::Session

Holds a token and the time it was issued, and reports whether it is still within the 10-unit freshness window.

## Behaviour

- Session("t") -> token is "t" and issued is now() at construction
- is_fresh is True while less than 10 units of now() have passed since issued
- Each instance records its own issued time, so two instances created at different times have different windows

## Terms

auth session, login state, token holder, freshness window, session object, expiry check, issued timestamp

## Connections

- **Location:** `tests/fixtures/repos/gate0/py/app/session.py:4-10`
- **Signature:** `class Session`
- **In:** [[file-tests-fixtures-repos-gate0-py-app-session-py]]
- **Called by:** [[session-refresh]]
