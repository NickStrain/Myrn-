---
id: db-set-meta
type: summary
status: unverified
title: Db::set_meta
source: myrn summarize
about: sym:crates/myrn-store/src/db.rs#db::Db::set_meta
anchors:
- sym:crates/myrn-store/src/db.rs#db::Db::set_meta@a323de1a8170b8f1
---
# Db::set_meta

Inserts key and value into the meta table, or overwrites the value if the key already exists. It does not report whether a row was replaced.

## Connections

- **Location:** `crates/myrn-store/src/db.rs:70-77`
- **Signature:** `pub fn set_meta(&self, key: &str, value: &str) -> Result<()>`
- **In:** [[file-crates-myrn-store-src-db-rs]]
- **Tests:** `migrates_and_stores_meta`
