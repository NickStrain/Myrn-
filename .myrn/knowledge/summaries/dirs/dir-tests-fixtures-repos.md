---
id: dir-tests-fixtures-repos
type: summary
status: unverified
title: tests/fixtures/repos/
source: myrn summarize
about: module:tests/fixtures/repos
anchors:
- module:tests/fixtures/repos@f5ef3e7339f8ee84
---
# tests/fixtures/repos/

Fixture repo for exercising token refresh logic across three parallel implementations: py/ (Session(token, issued_at) with refresh(token_store, token), backed by an unbounded list-based TokenStore and a stubbed now()), src/ (a Rust gate0 crate with the same model, where refresh is re-exported at the crate root), and web/src/ (a TypeScript Session with a 10-unit freshness window, using a TokenStore imported from ./store). Each implementation has a success-path test for token "abc" (tests/test_session.py::test_refresh_known_token, tests/refresh.rs, web/session.test.ts); only the Rust session module has an unknown-token unit test, so the expired-token path and the integration and web unknown-token paths are untested.

## Connections

- **In:** [[dir-tests-fixtures]]
- **Directories:** [[dir-tests-fixtures-repos-gate0]]
