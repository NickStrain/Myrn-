---
id: db-get-meta
type: summary
status: unverified
title: Db::get_meta
source: myrn summarize
about: sym:crates/myrn-store/src/db.rs#db::Db::get_meta
anchors:
- sym:crates/myrn-store/src/db.rs#db::Db::get_meta@4027d95f24198800
---
# Db::get_meta

Returns the value stored under key in the meta table, or None if the key is absent. Returns an error on SQL failure.

## Connections

- **Location:** `crates/myrn-store/src/db.rs:63-68`
- **Signature:** `pub fn get_meta(&self, key: &str) -> Result<Option<String>>`
- **In:** [[file-crates-myrn-store-src-db-rs]]
- **Called by:** [[engine-status]]
- **Tests:** `migrates_and_stores_meta`
