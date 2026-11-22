---
id: dir-tests-fixtures-repos-gate0-web-src
type: summary
status: unverified
title: tests/fixtures/repos/gate0/web/src/
source: myrn summarize
about: module:tests/fixtures/repos/gate0/web/src
anchors:
- module:tests/fixtures/repos/gate0/web/src@d5be09cacd2fa72a
---
# tests/fixtures/repos/gate0/web/src/

Auth/session fixture for the gate0 web app: store.ts holds an in-memory TokenStore that records issued tokens and answers whether a token was issued, plus a fixed-value now() helper. session.ts defines a Session class (token plus issue timestamp, 10-unit freshness window) and refresh(), which checks a token against a TokenStore (imported from ./store, along with now()) and returns a fresh Session. session.test.ts has one test that checks refresh() returns a defined value for the known token "abc"; unknown and expired tokens are not covered.

## Connections

- **In:** [[dir-tests-fixtures-repos-gate0-web]]
- **Files:** [[file-tests-fixtures-repos-gate0-web-src-session-test-ts]], [[file-tests-fixtures-repos-gate0-web-src-session-ts]], [[file-tests-fixtures-repos-gate0-web-src-store-ts]]
