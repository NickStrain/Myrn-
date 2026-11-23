---
id: db-schema-version
type: summary
status: unverified
title: Db::schema_version
source: myrn summarize
about: sym:crates/myrn-store/src/db.rs#db::Db::schema_version
anchors:
- sym:crates/myrn-store/src/db.rs#db::Db::schema_version@bd24cc99f1a2d619
map_version: '2'
---
# Db::schema_version

Reads PRAGMA user_version from the database and returns it as an i64.

## Behaviour

- fresh database -> 0, so every migration is pending
- after migration i is applied -> i + 1 (the migration count)
- no arguments; returns error if the pragma query fails

## Terms

user_version, schema number, migration level, database version, schema level

## Connections

- **Location:** `crates/myrn-store/src/db.rs:44-48`
- **Signature:** `pub fn schema_version(&self) -> Result<i64>`
- **In:** [[file-crates-myrn-store-src-db-rs]]
- **Called by:** [[engine-doctor]], [[db-migrate]]
- **Tests:** `migrates_and_stores_meta`
