---
id: db-migrate
type: summary
status: unverified
title: Db::migrate
source: myrn summarize
about: sym:crates/myrn-store/src/db.rs#db::Db::migrate
anchors:
- sym:crates/myrn-store/src/db.rs#db::Db::migrate@79ff68567de28753
map_version: '2'
---
# Db::migrate

Applies pending migrations in order, each in a BEGIN/COMMIT with user_version set to its index plus one, and refuses databases newer than this binary.

## Behaviour

- stored version greater than SCHEMA_VERSION -> error asking the user to upgrade myrn or run myrn rebuild
- stored version 3 with 7 migrations -> runs migrations 4 to 7 (indices 3 to 6) and sets version to 7
- a failing migration returns an error, and this code issues no ROLLBACK

## Terms

schema upgrade, apply migrations, run migrations, schema update, forward only, downgrade refused

## Connections

- **Location:** `crates/myrn-store/src/db.rs:50-65`
- **Signature:** `fn migrate(&self) -> Result<()>`
- **In:** [[file-crates-myrn-store-src-db-rs]]
- **Calls:** [[db-schema-version]]
- **Called by:** [[db-setup]]
