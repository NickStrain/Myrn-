---
id: dedupe-load
type: summary
status: unverified
title: dedupe::load
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/dedupe.rs#distill::dedupe::load
anchors:
- sym:crates/myrn-engine/src/distill/dedupe.rs#distill::dedupe::load@10ea19d5b020fe57
map_version: '2'
---
# dedupe::load

Runs a SQL query with bound arguments and turns each row into an Existing, skipping rows whose JSON does not parse as a Note.

## Behaviour

- row whose data fails serde_json parsing -> silently dropped, not an error
- no matching rows -> empty Vec
- SQL must select path first and data second, since column 0 becomes the key

## Terms

query helper, row decode, json parse, skip corrupt rows, notes query, bind arguments

## Connections

- **Location:** `crates/myrn-engine/src/distill/dedupe.rs:28-41`
- **Signature:** `fn load(conn: &Connection, sql: &str, args: &[&dyn rusqlite::ToSql]) -> Result<Vec<Existing>>`
- **In:** [[file-crates-myrn-engine-src-distill-dedupe-rs]]
- **Called by:** [[dedupe-find-bug]], [[dedupe-find-summary]], [[dedupe-find-id]]
