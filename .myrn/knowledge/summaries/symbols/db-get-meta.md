---
id: db-get-meta
type: summary
status: unverified
title: Db::get_meta
source: myrn summarize
about: sym:crates/myrn-store/src/db.rs#db::Db::get_meta
anchors:
- sym:crates/myrn-store/src/db.rs#db::Db::get_meta@4027d95f24198800
map_version: '2'
---
# Db::get_meta

Looks up a value by key in the meta table and returns it as Option<String>.

## Behaviour

- existing key -> Some(value)
- missing key -> None, not an error
- empty string key -> looked up like any other key

## Terms

read metadata, key value lookup, meta table, stored setting, fetch setting

## Connections

- **Location:** `crates/myrn-store/src/db.rs:67-72`
- **Signature:** `pub fn get_meta(&self, key: &str) -> Result<Option<String>>`
- **In:** [[file-crates-myrn-store-src-db-rs]]
- **Called by:** [[engine-status]]
- **Tests:** `migrates_and_stores_meta`
