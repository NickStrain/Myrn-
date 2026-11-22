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

Defines the auth Session type, which holds a token string and the issued timestamp, and a refresh function that looks a token up in a TokenStore and returns a fresh Session for it. Also includes a single unit test for the unknown-token case.

## Connections

- **Location:** `tests/fixtures/repos/gate0/src/auth/session.rs`
- **In:** [[dir-tests-fixtures-repos-gate0-src-auth]]
- **Defines:** [[session-session-session]], [[session-new]], [[session-is-fresh-session]], [[session-refresh-session]]
- **Imports:** [[file-tests-fixtures-repos-gate0-src-store-rs]]
- **Imported by:** [[file-tests-fixtures-repos-gate0-src-auth-mod-rs]], [[file-tests-fixtures-repos-gate0-src-lib-rs]]
