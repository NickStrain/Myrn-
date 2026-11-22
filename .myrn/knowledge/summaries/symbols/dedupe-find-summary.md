---
id: dedupe-find-summary
type: summary
status: unverified
title: dedupe::find_summary
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/dedupe.rs#distill::dedupe::find_summary
anchors:
- sym:crates/myrn-engine/src/distill/dedupe.rs#distill::dedupe::find_summary@9c2aac919ef7d438
---
# dedupe::find_summary

Returns the first summary note (shared first, then by path) with an anchor whose symbol_id equals target, or None. Matches on symbol id only.

## Connections

- **Location:** `crates/myrn-engine/src/distill/dedupe.rs:99-109`
- **Signature:** `pub fn find_summary(conn: &Connection, target: &str) -> Result<Option<Existing>>`
- **In:** [[file-crates-myrn-engine-src-distill-dedupe-rs]]
- **Calls:** [[dedupe-load]]
- **Called by:** [[engine-record]]
