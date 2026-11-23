---
id: dedupe-find-id
type: summary
status: unverified
title: dedupe::find_id
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/dedupe.rs#distill::dedupe::find_id
anchors:
- sym:crates/myrn-engine/src/distill/dedupe.rs#distill::dedupe::find_id@e50a4887964831c3
map_version: '2'
---
# dedupe::find_id

Returns the note whose id equals the given string exactly, preferring shared-visibility notes and then the lowest path.

## Behaviour

- id not present -> None
- same id in several rows -> shared row first, then lowest path
- exact string equality with no trimming or case folding

## Terms

content hash, exact match, note id, lookup by id, identical content, duplicate id

## Connections

- **Location:** `crates/myrn-engine/src/distill/dedupe.rs:112-121`
- **Signature:** `pub fn find_id(conn: &Connection, id: &str) -> Result<Option<Existing>>`
- **In:** [[file-crates-myrn-engine-src-distill-dedupe-rs]]
- **Calls:** [[dedupe-load]]
- **Called by:** [[engine-record]], [[engine-forget]], [[engine-publish-plan]], [[engine-gc]], [[engine-confirm]], [[engine-supersede]], [[engine-promote]]
