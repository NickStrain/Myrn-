---
id: file-crates-myrn-store-src-db-rs
type: summary
status: unverified
title: crates/myrn-store/src/db.rs
source: myrn summarize
about: file:crates/myrn-store/src/db.rs
anchors:
- file:crates/myrn-store/src/db.rs@4d76372cccc3a7a3
---
# crates/myrn-store/src/db.rs

Opens the memory.db SQLite database and brings its schema up to date. It embeds three migration SQL files, defines SCHEMA_VERSION as their count, and provides the Db wrapper with open helpers, schema version and migration logic, a key-value meta table accessor, and a quick integrity check.

## Connections

- **Location:** `crates/myrn-store/src/db.rs`
- **In:** [[dir-crates-myrn-store-src]]
- **Defines:** [[db-db]], [[db-open]], [[db-open-in-memory]], [[db-setup]], [[db-schema-version]], [[db-migrate]], [[db-get-meta]], [[db-set-meta]], [[db-integrity-ok]]
- **Imported by:** [[file-crates-myrn-store-src-lib-rs]]
