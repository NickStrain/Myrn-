---
id: file-tests-fixtures-repos-gate0-py-tests-test-session-py
type: summary
status: unverified
title: tests/fixtures/repos/gate0/py/tests/test_session.py
source: myrn summarize
about: file:tests/fixtures/repos/gate0/py/tests/test_session.py
anchors:
- file:tests/fixtures/repos/gate0/py/tests/test_session.py@803a56e8d395ead8
---
# tests/fixtures/repos/gate0/py/tests/test_session.py

Test for the session refresh path: seeds a TokenStore with token "abc" and asserts that refresh(store, "abc") returns a non-None result. Imports TokenStore from app.store and refresh from app.session; the only test here is test_refresh_known_token.

## Connections

- **Location:** `tests/fixtures/repos/gate0/py/tests/test_session.py`
- **In:** [[dir-tests-fixtures-repos-gate0-py-tests]]
- **Imports:** [[file-tests-fixtures-repos-gate0-py-app-session-py]], [[file-tests-fixtures-repos-gate0-py-app-store-py]]
