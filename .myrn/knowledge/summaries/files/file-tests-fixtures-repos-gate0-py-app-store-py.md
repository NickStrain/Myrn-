---
id: file-tests-fixtures-repos-gate0-py-app-store-py
type: summary
status: unverified
title: tests/fixtures/repos/gate0/py/app/store.py
source: myrn summarize
about: file:tests/fixtures/repos/gate0/py/app/store.py
anchors:
- file:tests/fixtures/repos/gate0/py/app/store.py@f79e5d7bcc79fb86
---
# tests/fixtures/repos/gate0/py/app/store.py

Defines an in-memory token registry: the TokenStore class keeps an unbounded list of issued tokens with membership lookup and append, plus a stub now() helper that returns a fixed value.

## Connections

- **Location:** `tests/fixtures/repos/gate0/py/app/store.py`
- **In:** [[dir-tests-fixtures-repos-gate0-py-app]]
- **Defines:** [[store-tokenstore]], [[tokenstore-init]], [[tokenstore-lookup]], [[tokenstore-insert]], [[store-now]]
- **Imported by:** [[file-tests-fixtures-repos-gate0-py-app-session-py]], [[file-tests-fixtures-repos-gate0-py-tests-test-session-py]]
