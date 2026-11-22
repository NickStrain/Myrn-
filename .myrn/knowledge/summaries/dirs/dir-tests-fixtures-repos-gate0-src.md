---
id: dir-tests-fixtures-repos-gate0-src
type: summary
status: unverified
title: tests/fixtures/repos/gate0/src/
source: myrn summarize
about: module:tests/fixtures/repos/gate0/src
anchors:
- module:tests/fixtures/repos/gate0/src@6b2421d4fad3d284
---
# tests/fixtures/repos/gate0/src/

Fixture Rust library crate (gate0) with an in-memory auth/session model: store.rs holds TokenStore (a Vec<String> of issued tokens with new, lookup, insert) and a fixed-value now(); auth/mod.rs only declares the session submodule, where session.rs or session/mod.rs defines Session (token string plus issued timestamp) and refresh(), which looks the token up in a TokenStore and returns a new Session, with a unit test for the unknown-token case. lib.rs declares the auth and store modules and re-exports auth::session::refresh at the crate root.

## Connections

- **In:** [[dir-tests-fixtures-repos-gate0]]
- **Directories:** [[dir-tests-fixtures-repos-gate0-src-auth]]
- **Files:** [[file-tests-fixtures-repos-gate0-src-lib-rs]], [[file-tests-fixtures-repos-gate0-src-store-rs]]
