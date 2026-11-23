---
id: dir-tests-fixtures-repos-gate0-web
type: summary
status: unverified
title: tests/fixtures/repos/gate0/web/
source: myrn summarize
about: module:tests/fixtures/repos/gate0/web
anchors:
- module:tests/fixtures/repos/gate0/web@14cce511bad7079d
---
# tests/fixtures/repos/gate0/web/

Test fixture for a small session-token module in the gate0 web app, used as a sample repo for code-map and agent tooling rather than production code. src/session.ts defines the Session class and refresh(), which looks up a token in the in-memory TokenStore (store.ts) and returns a fresh Session or undefined; session.test.ts has a single test covering the known token 'abc' only, with unknown and expired tokens untested and now() in store.ts hardcoded to return 42.

## Connections

- **In:** [[dir-tests-fixtures-repos-gate0]]
- **Directories:** [[dir-tests-fixtures-repos-gate0-web-src]]
