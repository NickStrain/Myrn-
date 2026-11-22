---
id: dir-tests-fixtures-repos-gate0-py-tests
type: summary
status: unverified
title: tests/fixtures/repos/gate0/py/tests/
source: myrn summarize
about: module:tests/fixtures/repos/gate0/py/tests
anchors:
- module:tests/fixtures/repos/gate0/py/tests@50ed0ab10e195859
---
# tests/fixtures/repos/gate0/py/tests/

Test directory for the session refresh path under tests/fixtures/repos/gate0/py. It holds a single test, test_session.py::test_refresh_known_token, which seeds a TokenStore (from app.store) with token "abc" and asserts that app.session.refresh(store, "abc") returns a non-None result.

## Connections

- **In:** [[dir-tests-fixtures-repos-gate0-py]]
- **Files:** [[file-tests-fixtures-repos-gate0-py-tests-test-session-py]]
