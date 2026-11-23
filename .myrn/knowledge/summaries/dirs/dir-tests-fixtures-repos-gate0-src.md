---
id: dir-tests-fixtures-repos-gate0-src
type: summary
status: unverified
title: tests/fixtures/repos/gate0/src/
source: myrn summarize
about: module:tests/fixtures/repos/gate0/src
anchors:
- module:tests/fixtures/repos/gate0/src@8209db5ecad985e6
---
# tests/fixtures/repos/gate0/src/

Source root of the gate0 fixture crate. lib.rs declares the auth and store modules and re-exports auth::session::refresh at the crate root. store.rs holds an in-memory Vec-of-strings token store with new, lookup, and add methods, plus a standalone now() that always returns 42. auth/ holds the session logic: mod.rs only declares the session submodule, and the Session type, its freshness check, and refresh() (which looks up a token in a TokenStore and returns a fresh Session or None) live in session.rs or session/mod.rs, which were not shown.

## Connections

- **In:** [[dir-tests-fixtures-repos-gate0]]
- **Directories:** [[dir-tests-fixtures-repos-gate0-src-auth]]
- **Files:** [[file-tests-fixtures-repos-gate0-src-lib-rs]], [[file-tests-fixtures-repos-gate0-src-store-rs]]
