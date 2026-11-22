---
id: db-open
type: summary
status: unverified
title: Db::open
source: myrn summarize
about: sym:crates/myrn-store/src/db.rs#db::Db::open
anchors:
- sym:crates/myrn-store/src/db.rs#db::Db::open@3da681c2e6801118
---
# Db::open

Opens or creates the SQLite file at path, adding the path to any open error, then calls setup. Fails if the file's schema version is newer than this binary supports.

## Connections

- **Location:** `crates/myrn-store/src/db.rs:22-25`
- **Signature:** `pub fn open(path: &Path) -> Result<Db>`
- **In:** [[file-crates-myrn-store-src-db-rs]]
- **Calls:** [[db-setup]]
- **Called by:** [[engine-open-located]], [[engine-rebuild]]
