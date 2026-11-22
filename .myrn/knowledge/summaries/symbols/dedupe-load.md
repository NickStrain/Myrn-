---
id: dedupe-load
type: summary
status: unverified
title: dedupe::load
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/dedupe.rs#distill::dedupe::load
anchors:
- sym:crates/myrn-engine/src/distill/dedupe.rs#distill::dedupe::load@10ea19d5b020fe57
---
# dedupe::load

Runs the given SQL with positional args, expects two columns (key, JSON data), and returns each row that deserializes as a Note; rows that fail to parse are silently skipped, not errors.

## Connections

- **Location:** `crates/myrn-engine/src/distill/dedupe.rs:28-41`
- **Signature:** `fn load(conn: &Connection, sql: &str, args: &[&dyn rusqlite::ToSql]) -> Result<Vec<Existing>>`
- **In:** [[file-crates-myrn-engine-src-distill-dedupe-rs]]
- **Called by:** [[dedupe-find-bug]], [[dedupe-find-summary]], [[dedupe-find-id]]
