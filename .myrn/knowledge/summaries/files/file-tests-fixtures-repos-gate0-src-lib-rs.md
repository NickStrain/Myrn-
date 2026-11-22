---
id: file-tests-fixtures-repos-gate0-src-lib-rs
type: summary
status: unverified
title: tests/fixtures/repos/gate0/src/lib.rs
source: myrn summarize
about: file:tests/fixtures/repos/gate0/src/lib.rs
anchors:
- file:tests/fixtures/repos/gate0/src/lib.rs@7236af2469001dfe
---
# tests/fixtures/repos/gate0/src/lib.rs

Crate root for the gate0 fixture library. It declares the public `auth` and `store` modules and re-exports `auth::session::refresh` at the crate root, so callers can use `refresh` directly without the `auth::session` path.

## Connections

- **Location:** `tests/fixtures/repos/gate0/src/lib.rs`
- **In:** [[dir-tests-fixtures-repos-gate0-src]]
- **Imports:** [[file-tests-fixtures-repos-gate0-src-auth-mod-rs]], [[file-tests-fixtures-repos-gate0-src-auth-session-rs]], [[file-tests-fixtures-repos-gate0-src-store-rs]]
