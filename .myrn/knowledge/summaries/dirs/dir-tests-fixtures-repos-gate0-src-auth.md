---
id: dir-tests-fixtures-repos-gate0-src-auth
type: summary
status: unverified
title: tests/fixtures/repos/gate0/src/auth/
source: myrn summarize
about: module:tests/fixtures/repos/gate0/src/auth
anchors:
- module:tests/fixtures/repos/gate0/src/auth@13f6dd7db2a81e37
---
# tests/fixtures/repos/gate0/src/auth/

Auth module directory for the gate0 fixture repo. mod.rs only declares the `session` submodule; the real logic is in session.rs (or session/mod.rs, not shown here), which defines the Session type (token plus issue timestamp), its constructor and freshness check, and refresh(), which looks up a token in a TokenStore and returns a fresh Session or None.

## Connections

- **In:** [[dir-tests-fixtures-repos-gate0-src]]
- **Files:** [[file-tests-fixtures-repos-gate0-src-auth-mod-rs]], [[file-tests-fixtures-repos-gate0-src-auth-session-rs]]
