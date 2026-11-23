---
id: file-tests-fixtures-repos-gate0-src-auth-session-rs
type: summary
status: unverified
title: tests/fixtures/repos/gate0/src/auth/session.rs
source: myrn summarize
about: file:tests/fixtures/repos/gate0/src/auth/session.rs
anchors:
- file:tests/fixtures/repos/gate0/src/auth/session.rs@51c2693cadc05f5c
---
# tests/fixtures/repos/gate0/src/auth/session.rs

Defines the auth Session type (token plus issue timestamp), its constructor and freshness check, and refresh(), which looks up a token in a TokenStore and returns a fresh Session or None. Includes one unit test for an unknown token.

## Connections

- **Location:** `tests/fixtures/repos/gate0/src/auth/session.rs`
- **In:** [[dir-tests-fixtures-repos-gate0-src-auth]]
- **Defines:** [[session-session-session]], [[session-new]], [[session-is-fresh-session]], [[session-refresh-session]]
- **Imports:** [[file-tests-fixtures-repos-gate0-src-store-rs]]
- **Imported by:** [[file-tests-fixtures-repos-gate0-src-auth-mod-rs]], [[file-tests-fixtures-repos-gate0-src-lib-rs]]
