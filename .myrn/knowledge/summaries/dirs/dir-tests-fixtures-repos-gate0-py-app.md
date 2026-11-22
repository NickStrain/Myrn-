---
id: dir-tests-fixtures-repos-gate0-py-app
type: summary
status: unverified
title: tests/fixtures/repos/gate0/py/app/
source: myrn summarize
about: module:tests/fixtures/repos/gate0/py/app
anchors:
- module:tests/fixtures/repos/gate0/py/app@c29e0110d6eba0c2
---
# tests/fixtures/repos/gate0/py/app/

Fixture package for the gate0 Python test repo: it holds a small token-session model and an in-memory token store used to exercise refresh logic. session.py's Session(token, issued_at) reports freshness, and refresh(token_store, token) returns a new Session only if the store contains the token and the session is fresh; store.py's TokenStore keeps an unbounded list of issued tokens with append and membership checks, and now() is a stub returning a fixed value. __init__.py is empty and only marks the directory as a package.

## Connections

- **In:** [[dir-tests-fixtures-repos-gate0-py]]
- **Files:** [[file-tests-fixtures-repos-gate0-py-app-init-py]], [[file-tests-fixtures-repos-gate0-py-app-session-py]], [[file-tests-fixtures-repos-gate0-py-app-store-py]]
