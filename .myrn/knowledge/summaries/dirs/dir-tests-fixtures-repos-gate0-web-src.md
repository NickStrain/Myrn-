---
id: dir-tests-fixtures-repos-gate0-web-src
type: summary
status: unverified
title: tests/fixtures/repos/gate0/web/src/
source: myrn summarize
about: module:tests/fixtures/repos/gate0/web/src
anchors:
- module:tests/fixtures/repos/gate0/web/src@6154253efc9c6db0
---
# tests/fixtures/repos/gate0/web/src/

Test fixture sources for a small session-token module in the gate0 web app. session.ts defines the Session class (token, issue time, freshness check) and refresh(), which looks up a token in the in-memory TokenStore from store.ts and returns a fresh Session or undefined. session.test.ts has one test that checks refresh() returns a defined value for the known token 'abc'; unknown and expired tokens are not covered, and now() in store.ts is hardcoded to return 42.

## Connections

- **In:** [[dir-tests-fixtures-repos-gate0-web]]
- **Files:** [[file-tests-fixtures-repos-gate0-web-src-session-test-ts]], [[file-tests-fixtures-repos-gate0-web-src-session-ts]], [[file-tests-fixtures-repos-gate0-web-src-store-ts]]
