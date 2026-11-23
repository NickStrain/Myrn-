---
id: log-record
type: summary
status: unverified
title: log::record
source: myrn summarize
about: sym:crates/myrn-engine/src/pack/log.rs#pack::log::record
anchors:
- sym:crates/myrn-engine/src/pack/log.rs#pack::log::record@b9ba60db65e598a3
map_version: '2'
---
# log::record

Inserts one usage row per note ID with injected=1, used=0, the note's current status, and the current unix time; the session ID is bound to every row.

## Behaviour

- ids = ["a","b"] -> two INSERT rows, one per id, each with session_id = session
- ids empty -> no rows written, returns Ok(())
- id not present in nodes -> status subquery yields NULL, row is still inserted with status NULL

## Terms

injection log, usage tracking, attribution, note injection, injected notes, session usage, status snapshot, audit row

## Connections

- **Location:** `crates/myrn-engine/src/pack/log.rs:6-15`
- **Signature:** `pub fn record(conn: &Connection, session: &str, ids: &[String]) -> Result<()>`
- **In:** [[file-crates-myrn-engine-src-pack-log-rs]]
- **Called by:** [[engine-log-injection]]
