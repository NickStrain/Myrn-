---
id: dir-tests-fixtures-repos-gate0
type: summary
status: unverified
title: tests/fixtures/repos/gate0/
source: myrn summarize
about: module:tests/fixtures/repos/gate0
anchors:
- module:tests/fixtures/repos/gate0@378a52b4f4e8d9a6
---
# tests/fixtures/repos/gate0/

Fixture repo for exercising token refresh logic across three parallel implementations: py/ (Session(token, issued_at), refresh(token_store, token), and an unbounded list-backed TokenStore with a stubbed now()), src/ (a Rust gate0 crate with the same model, where refresh is re-exported at the crate root), and web/src/ (TypeScript Session with a 10-unit freshness window and a TokenStore imported from ./store). Each has a success-path test for token "abc" (tests/test_session.py::test_refresh_known_token, tests/refresh.rs, web/session.test.ts), and only the Rust session module has an unknown-token unit test, so the expired-token path and the integration and web unknown-token paths are uncovered.

## Connections

- **In:** [[dir-tests-fixtures-repos]]
- **Directories:** [[dir-tests-fixtures-repos-gate0-py]], [[dir-tests-fixtures-repos-gate0-src]], [[dir-tests-fixtures-repos-gate0-tests]], [[dir-tests-fixtures-repos-gate0-web]]
