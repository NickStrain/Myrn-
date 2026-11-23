---
id: db-setup
type: summary
status: unverified
title: Db::setup
source: myrn summarize
about: sym:crates/myrn-store/src/db.rs#db::Db::setup
anchors:
- sym:crates/myrn-store/src/db.rs#db::Db::setup@c805c5ec961a97bb
map_version: '2'
---
# Db::setup

Sets busy timeout and WAL journal mode with NORMAL synchronous, wraps the connection in Db, then runs migrate.

## Behaviour

- busy lock held longer than 5 seconds -> the waiting statement fails
- in-memory connections report WAL mode through the pragma, but WAL only applies to file databases
- any pragma or migrate error -> returned and the Db is dropped

## Terms

initialize connection, pragmas, wal mode, busy timeout, connection setup, journal mode

## Connections

- **Location:** `crates/myrn-store/src/db.rs:35-42`
- **Signature:** `fn setup(conn: Connection) -> Result<Db>`
- **In:** [[file-crates-myrn-store-src-db-rs]]
- **Calls:** [[db-migrate]]
- **Called by:** [[db-open]], [[db-open-in-memory]]
