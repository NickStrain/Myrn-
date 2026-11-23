---
id: db-set-meta
type: summary
status: unverified
title: Db::set_meta
source: myrn summarize
about: sym:crates/myrn-store/src/db.rs#db::Db::set_meta
anchors:
- sym:crates/myrn-store/src/db.rs#db::Db::set_meta@a323de1a8170b8f1
map_version: '2'
---
# Db::set_meta

Inserts a key/value pair into the meta table, or overwrites the value if the key already exists.

## Behaviour

- new key -> row inserted
- existing key with new value -> value replaced, one row per key
- returns Ok(()) on success and the SQL error on failure

## Terms

write metadata, upsert, store setting, save key value, update meta

## Connections

- **Location:** `crates/myrn-store/src/db.rs:74-81`
- **Signature:** `pub fn set_meta(&self, key: &str, value: &str) -> Result<()>`
- **In:** [[file-crates-myrn-store-src-db-rs]]
- **Tests:** `migrates_and_stores_meta`
