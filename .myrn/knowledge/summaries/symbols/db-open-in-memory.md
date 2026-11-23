---
id: db-open-in-memory
type: summary
status: unverified
title: Db::open_in_memory
source: myrn summarize
about: sym:crates/myrn-store/src/db.rs#db::Db::open_in_memory
anchors:
- sym:crates/myrn-store/src/db.rs#db::Db::open_in_memory@e3d4d010301f3b18
map_version: '2'
---
# Db::open_in_memory

Creates an in-memory SQLite connection and runs setup on it, so nothing is written to disk.

## Behaviour

- no arguments -> fresh empty Db at schema version SCHEMA_VERSION
- each call gives a separate empty database with no shared state

## Terms

ephemeral database, temp database, ram database, test database, transient storage

## Connections

- **Location:** `crates/myrn-store/src/db.rs:31-33`
- **Signature:** `pub fn open_in_memory() -> Result<Db>`
- **In:** [[file-crates-myrn-store-src-db-rs]]
- **Calls:** [[db-setup]]
- **Called by:** [[engine-rebuild]]
- **Tests:** `migrates_and_stores_meta`, `neighbours_both_ways`
