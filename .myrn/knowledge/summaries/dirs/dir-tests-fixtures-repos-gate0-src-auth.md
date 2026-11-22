---
id: dir-tests-fixtures-repos-gate0-src-auth
type: summary
status: unverified
title: tests/fixtures/repos/gate0/src/auth/
source: myrn summarize
about: module:tests/fixtures/repos/gate0/src/auth
anchors:
- module:tests/fixtures/repos/gate0/src/auth@1f9aa25b89f352ae
---
# tests/fixtures/repos/gate0/src/auth/

Auth package root for tests/fixtures/repos/gate0/src/auth/. mod.rs only declares the `session` submodule; the session logic (a Session type holding a token string and issued timestamp, plus a refresh function that looks the token up in a TokenStore and returns a fresh Session) lives in session.rs or session/mod.rs, with a unit test covering the unknown-token case.

## Connections

- **In:** [[dir-tests-fixtures-repos-gate0-src]]
- **Files:** [[file-tests-fixtures-repos-gate0-src-auth-mod-rs]], [[file-tests-fixtures-repos-gate0-src-auth-session-rs]]
