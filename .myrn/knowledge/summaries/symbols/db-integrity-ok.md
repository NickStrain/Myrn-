---
id: db-integrity-ok
type: summary
status: unverified
title: Db::integrity_ok
source: myrn summarize
about: sym:crates/myrn-store/src/db.rs#db::Db::integrity_ok
anchors:
- sym:crates/myrn-store/src/db.rs#db::Db::integrity_ok@7e3502f20351c019
---
# Db::integrity_ok

Runs PRAGMA quick_check and returns true only if the first result row is exactly "ok". Corruption is reported as false rather than as an error, and only the first row is read.

## Connections

- **Location:** `crates/myrn-store/src/db.rs:79-84`
- **Signature:** `pub fn integrity_ok(&self) -> Result<bool>`
- **In:** [[file-crates-myrn-store-src-db-rs]]
- **Called by:** [[engine-doctor]]
- **Tests:** `migrates_and_stores_meta`
