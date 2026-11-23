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

Session token helpers for the app package: defines the Session class, which records a token and its creation time with a 10-unit freshness window, and the refresh function, which looks up a token in a store and issues a new Session for it. Time comes from now() imported from the store module.

## Connections

- **Location:** `tests/fixtures/repos/gate0/py/app/session.py`
- **In:** [[dir-tests-fixtures-repos-gate0-py-app]]
- **Defines:** [[session-session]], [[session-init]], [[session-is-fresh]], [[session-refresh]]
- **Imports:** [[file-tests-fixtures-repos-gate0-py-app-store-py]]
- **Imported by:** [[file-tests-fixtures-repos-gate0-py-tests-test-session-py]]
