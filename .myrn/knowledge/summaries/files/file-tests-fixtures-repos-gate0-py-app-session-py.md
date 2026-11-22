---
id: file-tests-fixtures-repos-gate0-py-app-session-py
type: summary
status: unverified
title: tests/fixtures/repos/gate0/py/app/session.py
source: myrn summarize
about: file:tests/fixtures/repos/gate0/py/app/session.py
anchors:
- file:tests/fixtures/repos/gate0/py/app/session.py@d9c6c71950e7d1c5
---
# tests/fixtures/repos/gate0/py/app/session.py

Defines a small session model and a refresh helper: Session records a token with its issue time and reports whether it is still fresh, and refresh looks up a token in a TokenStore and returns a new Session only if the lookup succeeds and the session is fresh.

## Connections

- **Location:** `tests/fixtures/repos/gate0/py/app/session.py`
- **In:** [[dir-tests-fixtures-repos-gate0-py-app]]
- **Defines:** [[session-session]], [[session-init]], [[session-is-fresh]], [[session-refresh]]
- **Imports:** [[file-tests-fixtures-repos-gate0-py-app-store-py]]
- **Imported by:** [[file-tests-fixtures-repos-gate0-py-tests-test-session-py]]
