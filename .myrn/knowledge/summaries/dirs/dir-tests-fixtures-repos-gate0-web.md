---
id: dir-tests-fixtures-repos-gate0-web
type: summary
status: unverified
title: tests/fixtures/repos/gate0/web/
source: myrn summarize
about: module:tests/fixtures/repos/gate0/web
anchors:
- module:tests/fixtures/repos/gate0/web@7c2067878f3ee31b
---
# tests/fixtures/repos/gate0/web/

Test fixture for the gate0 web app's auth/session logic, living under src/. store.ts provides an in-memory TokenStore that records issued tokens and answers whether a given token was issued, plus a fixed-value now() helper; session.ts defines a Session (token and issue timestamp with a 10-unit freshness window) and refresh(), which validates a token against a TokenStore imported from ./store and returns a fresh Session. session.test.ts has a single test asserting refresh() returns a defined value for token "abc"; unknown-token and expired-token paths are not covered.

## Connections

- **In:** [[dir-tests-fixtures-repos-gate0]]
- **Directories:** [[dir-tests-fixtures-repos-gate0-web-src]]
