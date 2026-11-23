---
id: file-crates-myrn-store-src-db-rs
type: summary
status: unverified
title: crates/myrn-store/src/db.rs
source: myrn summarize
about: file:crates/myrn-store/src/db.rs
anchors:
- file:crates/myrn-store/src/db.rs@ddc48660e4a11830
---
# crates/myrn-store/src/db.rs

Opens the memory.db SQLite file, or an in-memory database for tests, sets connection pragmas, and applies numbered schema migrations tracked by PRAGMA user_version. It also provides key-value storage in the meta table and a quick_check integrity test; the Db.conn field is public and exposes the raw rusqlite Connection.

## Connections

- **Location:** `crates/myrn-store/src/db.rs`
- **In:** [[dir-crates-myrn-store-src]]
- **Defines:** [[db-db]], [[db-open]], [[db-open-in-memory]], [[db-setup]], [[db-schema-version]], [[db-migrate]], [[db-get-meta]], [[db-set-meta]], [[db-integrity-ok]]
- **Imported by:** [[file-crates-myrn-store-src-lib-rs]]
