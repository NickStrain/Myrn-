---
id: dir-tests
type: summary
status: unverified
title: tests/
source: myrn summarize
about: module:tests
anchors:
- module:tests@3c85f3c75c60ef5f
---
# tests/

Contains fixtures/, a fixture repo that exercises token refresh across three parallel implementations under fixtures/repos/: py/ (Session(token, issued_at) with refresh(token_store, token) over an unbounded list-based TokenStore and a stubbed now()), src/ (a Rust gate0 crate with refresh re-exported at the crate root), and web/src/ (a TypeScript Session with a 10-unit freshness window and a TokenStore imported from ./store). Each has a success-path test for token "abc" (test_session.py::test_refresh_known_token, tests/refresh.rs, web/session.test.ts), but only the Rust session module has an unknown-token unit test, so the expired-token path and the integration and web unknown-token paths are uncovered.

## Connections

- **In:** [[project]]
- **Directories:** [[dir-tests-fixtures]]
