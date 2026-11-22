---
id: log-record
type: summary
status: unverified
title: log::record
source: myrn summarize
about: sym:crates/myrn-engine/src/pack/log.rs#pack::log::record
anchors:
- sym:crates/myrn-engine/src/pack/log.rs#pack::log::record@b9d64b9eae3c929d
---
# log::record

Takes a SQLite Connection, a session id, and a slice of note ids; inserts one `usage` row per id with injected=1 and used=0 using a cached prepared statement, returning an error on the first failed insert. Writes to the database only; no transaction is opened here, so a mid-loop failure leaves earlier rows committed.

## Connections

- **Location:** `crates/myrn-engine/src/pack/log.rs:6-14`
- **Signature:** `pub fn record(conn: &Connection, session: &str, ids: &[String]) -> Result<()>`
- **In:** [[file-crates-myrn-engine-src-pack-log-rs]]
- **Called by:** [[engine-log-injection]]
