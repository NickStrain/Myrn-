---
id: dedupe-find-id
type: summary
status: unverified
title: dedupe::find_id
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/dedupe.rs#distill::dedupe::find_id
anchors:
- sym:crates/myrn-engine/src/distill/dedupe.rs#distill::dedupe::find_id@e50a4887964831c3
---
# dedupe::find_id

Returns the first note whose id exactly equals the argument (shared first, then by path), or None.

## Connections

- **Location:** `crates/myrn-engine/src/distill/dedupe.rs:112-121`
- **Signature:** `pub fn find_id(conn: &Connection, id: &str) -> Result<Option<Existing>>`
- **In:** [[file-crates-myrn-engine-src-distill-dedupe-rs]]
- **Calls:** [[dedupe-load]]
- **Called by:** [[engine-record]]
