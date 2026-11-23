---
id: db-open
type: summary
status: unverified
title: Db::open
source: myrn summarize
about: sym:crates/myrn-store/src/db.rs#db::Db::open
anchors:
- sym:crates/myrn-store/src/db.rs#db::Db::open@3da681c2e6801118
map_version: '2'
---
# Db::open

Opens the SQLite file at the given path and passes the connection to setup.

## Behaviour

- path to existing or new file -> Db with pragmas set and migrations applied
- open failure -> error wrapped with context 'opening <path>'
- migration failure or newer schema -> error propagated from setup

## Terms

load database, file path, open file, database file, disk database

## Connections

- **Location:** `crates/myrn-store/src/db.rs:26-29`
- **Signature:** `pub fn open(path: &Path) -> Result<Db>`
- **In:** [[file-crates-myrn-store-src-db-rs]]
- **Calls:** [[db-setup]]
- **Called by:** [[engine-open-located]], [[engine-rebuild]]
