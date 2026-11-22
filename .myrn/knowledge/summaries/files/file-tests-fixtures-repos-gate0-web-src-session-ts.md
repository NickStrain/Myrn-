---
id: file-tests-fixtures-repos-gate0-web-src-session-ts
type: summary
status: unverified
title: tests/fixtures/repos/gate0/web/src/session.ts
source: myrn summarize
about: file:tests/fixtures/repos/gate0/web/src/session.ts
anchors:
- file:tests/fixtures/repos/gate0/web/src/session.ts@f188cb7b78aa5b9f
---
# tests/fixtures/repos/gate0/web/src/session.ts

Defines the Session class, which holds a token and an issue timestamp with a 10-unit freshness window, and the refresh function that checks a token against a TokenStore and returns a fresh Session. Imports TokenStore and now from ./store.

## Connections

- **Location:** `tests/fixtures/repos/gate0/web/src/session.ts`
- **In:** [[dir-tests-fixtures-repos-gate0-web-src]]
- **Defines:** [[session-3113]], [[session-constructor]], [[session-isfresh]], [[refresh]]
- **Imports:** [[file-tests-fixtures-repos-gate0-web-src-store-ts]]
- **Imported by:** [[file-tests-fixtures-repos-gate0-web-src-session-test-ts]]
