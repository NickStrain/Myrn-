---
id: db-migrate
type: summary
status: unverified
title: Db::migrate
source: myrn summarize
about: sym:crates/myrn-store/src/db.rs#db::Db::migrate
anchors:
- sym:crates/myrn-store/src/db.rs#db::Db::migrate@79ff68567de28753
---
# Db::migrate

Private. Fails if user_version is greater than SCHEMA_VERSION, with a message suggesting an upgrade or myrn rebuild. Otherwise, for each migration after the current version, runs it inside BEGIN/COMMIT with PRAGMA user_version set to its number, so the migration and version bump succeed or fail together. The migration files must not contain their own BEGIN/COMMIT, and the code has no explicit ROLLBACK on failure.

## Connections

- **Location:** `crates/myrn-store/src/db.rs:46-61`
- **Signature:** `fn migrate(&self) -> Result<()>`
- **In:** [[file-crates-myrn-store-src-db-rs]]
- **Calls:** [[db-schema-version]]
- **Called by:** [[db-setup]]
