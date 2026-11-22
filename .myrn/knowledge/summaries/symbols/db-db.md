---
id: db-db
type: summary
status: unverified
title: db::Db
source: myrn summarize
about: sym:crates/myrn-store/src/db.rs#db::Db
anchors:
- sym:crates/myrn-store/src/db.rs#db::Db@6ffbf5b37b49ad7a
---
# db::Db

Holds a public rusqlite Connection in field conn, so callers can run SQL on it directly. It is built only through setup, so every instance has the pragmas applied and migrations run.

## Connections

- **Location:** `crates/myrn-store/src/db.rs:17-19`
- **Signature:** `pub struct Db`
- **In:** [[file-crates-myrn-store-src-db-rs]]
