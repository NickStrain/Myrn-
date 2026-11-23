---
id: db-db
type: summary
status: unverified
title: db::Db
source: myrn summarize
about: sym:crates/myrn-store/src/db.rs#db::Db
anchors:
- sym:crates/myrn-store/src/db.rs#db::Db@6ffbf5b37b49ad7a
map_version: '2'
---
# db::Db

Holds the rusqlite Connection to memory.db in its public conn field.

## Behaviour

- Built only by setup, which runs pragmas and migrations first
- conn is pub, so code outside this module can use raw SQL and can build a Db without migrations

## Terms

database handle, connection wrapper, sqlite connection, memory db, db struct

## Connections

- **Location:** `crates/myrn-store/src/db.rs:21-23`
- **Signature:** `pub struct Db`
- **In:** [[file-crates-myrn-store-src-db-rs]]
