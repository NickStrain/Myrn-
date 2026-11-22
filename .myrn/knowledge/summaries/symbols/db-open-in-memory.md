---
id: db-open-in-memory
type: summary
status: unverified
title: Db::open_in_memory
source: myrn summarize
about: sym:crates/myrn-store/src/db.rs#db::Db::open_in_memory
anchors:
- sym:crates/myrn-store/src/db.rs#db::Db::open_in_memory@e3d4d010301f3b18
---
# Db::open_in_memory

Creates an in-memory database with the same setup and migrations as open. Its contents are discarded when the Db is dropped.

## Connections

- **Location:** `crates/myrn-store/src/db.rs:27-29`
- **Signature:** `pub fn open_in_memory() -> Result<Db>`
- **In:** [[file-crates-myrn-store-src-db-rs]]
- **Calls:** [[db-setup]]
- **Called by:** [[engine-rebuild]]
- **Tests:** `migrates_and_stores_meta`, `neighbours_both_ways`
