---
id: dir-tests-fixtures-repos-gate0-py-app
type: summary
status: unverified
title: tests/fixtures/repos/gate0/py/app/
source: myrn summarize
about: module:tests/fixtures/repos/gate0/py/app
anchors:
- module:tests/fixtures/repos/gate0/py/app@31e4c89d45e50a19
---
# tests/fixtures/repos/gate0/py/app/

Fixture Python package for the gate0 test repo: a minimal app with a session-token model (session.py), an in-memory token registry (store.py), and an empty __init__.py that only marks the directory as a package. session.py's refresh() looks up a token in a TokenStore and issues a new Session, and both modules get their timestamps from the now() helper in store.py.

## Connections

- **In:** [[dir-tests-fixtures-repos-gate0-py]]
- **Files:** [[file-tests-fixtures-repos-gate0-py-app-init-py]], [[file-tests-fixtures-repos-gate0-py-app-session-py]], [[file-tests-fixtures-repos-gate0-py-app-store-py]]
