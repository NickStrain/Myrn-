---
id: file-tests-fixtures-repos-gate0-src-store-rs
type: summary
status: unverified
title: tests/fixtures/repos/gate0/src/store.rs
source: myrn summarize
about: file:tests/fixtures/repos/gate0/src/store.rs
anchors:
- file:tests/fixtures/repos/gate0/src/store.rs@6906a87b1734fe7f
---
# tests/fixtures/repos/gate0/src/store.rs

Defines an in-memory token store: a TokenStore struct wrapping a Vec<String> of issued tokens, with constructor, lookup, and insert methods, plus a standalone now() function that returns a fixed value.

## Connections

- **Location:** `tests/fixtures/repos/gate0/src/store.rs`
- **In:** [[dir-tests-fixtures-repos-gate0-src]]
- **Defines:** [[store-tokenstore-store]], [[tokenstore-new]], [[tokenstore-lookup-store]], [[tokenstore-insert-store]], [[store-now-store]]
- **Imported by:** [[file-tests-fixtures-repos-gate0-src-auth-session-rs]], [[file-tests-fixtures-repos-gate0-src-lib-rs]], [[file-tests-fixtures-repos-gate0-tests-refresh-rs]]
