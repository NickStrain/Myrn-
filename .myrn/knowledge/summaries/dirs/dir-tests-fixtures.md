---
id: dir-tests-fixtures
type: summary
status: unverified
title: tests/fixtures/
source: myrn summarize
about: module:tests/fixtures
anchors:
- module:tests/fixtures@873dd01145ddf6da
---
# tests/fixtures/

Fixture repo for exercising token refresh logic across three parallel implementations under repos/: py/ (Session(token, issued_at) with refresh(token_store, token), using an unbounded list-based TokenStore and a stubbed now()), src/ (a Rust gate0 crate with the same model, with refresh re-exported at the crate root), and web/src/ (a TypeScript Session with a 10-unit freshness window and a TokenStore imported from ./store). Each has a success-path test for token "abc" (tests/test_session.py::test_refresh_known_token, tests/refresh.rs, web/session.test.ts), but only the Rust session module has an unknown-token unit test, so the expired-token path and the integration and web unknown-token paths have no coverage.

## Connections

- **In:** [[dir-tests]]
- **Directories:** [[dir-tests-fixtures-repos]]
