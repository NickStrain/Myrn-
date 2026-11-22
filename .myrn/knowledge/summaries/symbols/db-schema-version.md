---
id: db-schema-version
type: summary
status: unverified
title: Db::schema_version
source: myrn summarize
about: sym:crates/myrn-store/src/db.rs#db::Db::schema_version
anchors:
- sym:crates/myrn-store/src/db.rs#db::Db::schema_version@bd24cc99f1a2d619
---
# Db::schema_version

Returns PRAGMA user_version as an i64, which is the number of migrations already applied to the file.

## Connections

- **Location:** `crates/myrn-store/src/db.rs:40-44`
- **Signature:** `pub fn schema_version(&self) -> Result<i64>`
- **In:** [[file-crates-myrn-store-src-db-rs]]
- **Called by:** [[engine-doctor]], [[db-migrate]]
- **Tests:** `migrates_and_stores_meta`
