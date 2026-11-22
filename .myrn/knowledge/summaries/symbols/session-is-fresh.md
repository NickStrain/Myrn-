---
id: session-is-fresh
type: summary
status: unverified
title: Session::is_fresh
source: myrn summarize
about: sym:tests/fixtures/repos/gate0/py/app/session.py#tests.fixtures.repos.gate0.py.app.session.Session.is_fresh
anchors:
- sym:tests/fixtures/repos/gate0/py/app/session.py#tests.fixtures.repos.gate0.py.app.session.Session.is_fresh@9b6c959365e4e7c0
---
# Session::is_fresh

Returns True when issued + 10 is greater than now(), i.e. the session is less than 10 time units old; the window is hardcoded.

## Connections

- **Location:** `tests/fixtures/repos/gate0/py/app/session.py:9-10`
- **Signature:** `def is_fresh(self)`
- **In:** [[file-tests-fixtures-repos-gate0-py-app-session-py]]
- **Calls:** [[store-now]]
- **Called by:** [[session-refresh]]
