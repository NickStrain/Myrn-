---
id: dir-tests-fixtures-repos-gate0-py
type: summary
status: unverified
title: tests/fixtures/repos/gate0/py/
source: myrn summarize
about: module:tests/fixtures/repos/gate0/py
anchors:
- module:tests/fixtures/repos/gate0/py@8100a8be7c2a3628
---
# tests/fixtures/repos/gate0/py/

Fixture package for the gate0 Python test repo, used to exercise token refresh logic. app/session.py's Session(token, issued_at) reports freshness, and refresh(token_store, token) returns a new Session only when the store contains the token and the session is fresh; app/store.py's TokenStore is an unbounded in-memory list of issued tokens with append and membership checks, and now() is a stub returning a fixed value. tests/test_session.py::test_refresh_known_token seeds a TokenStore with "abc" and asserts refresh(store, "abc") is non-None.

## Connections

- **In:** [[dir-tests-fixtures-repos-gate0]]
- **Directories:** [[dir-tests-fixtures-repos-gate0-py-app]], [[dir-tests-fixtures-repos-gate0-py-tests]]
