---
id: db-setup
type: summary
status: unverified
title: Db::setup
source: myrn summarize
about: sym:crates/myrn-store/src/db.rs#db::Db::setup
anchors:
- sym:crates/myrn-store/src/db.rs#db::Db::setup@c805c5ec961a97bb
---
# Db::setup

Sets a 5-second busy timeout, switches journal_mode to WAL and synchronous to NORMAL, then runs migrate. This is the only constructor path, so all openers get these settings.

## Connections

- **Location:** `crates/myrn-store/src/db.rs:31-38`
- **Signature:** `fn setup(conn: Connection) -> Result<Db>`
- **In:** [[file-crates-myrn-store-src-db-rs]]
- **Calls:** [[db-migrate]]
- **Called by:** [[db-open]], [[db-open-in-memory]]
