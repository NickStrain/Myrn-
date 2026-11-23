---
id: db-integrity-ok
type: summary
status: unverified
title: Db::integrity_ok
source: myrn summarize
about: sym:crates/myrn-store/src/db.rs#db::Db::integrity_ok
anchors:
- sym:crates/myrn-store/src/db.rs#db::Db::integrity_ok@7e3502f20351c019
map_version: '2'
---
# Db::integrity_ok

Runs PRAGMA quick_check and returns true only if the first result row is exactly 'ok'.

## Behaviour

- healthy database -> Ok(true)
- corrupt database -> Ok(false) if the first row is not 'ok'
- only the first row is read, so later corruption messages are ignored

## Terms

corruption check, health check, quick check, database integrity, file corruption

## Connections

- **Location:** `crates/myrn-store/src/db.rs:83-88`
- **Signature:** `pub fn integrity_ok(&self) -> Result<bool>`
- **In:** [[file-crates-myrn-store-src-db-rs]]
- **Called by:** [[engine-doctor]]
- **Tests:** `migrates_and_stores_meta`
