---
id: dedupe-find-summary
type: summary
status: unverified
title: dedupe::find_summary
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/dedupe.rs#distill::dedupe::find_summary
anchors:
- sym:crates/myrn-engine/src/distill/dedupe.rs#distill::dedupe::find_summary@9c2aac919ef7d438
map_version: '2'
---
# dedupe::find_summary

Returns one summary note whose id is anchored to the given symbol id, preferring shared-visibility notes and then the lowest path.

## Behaviour

- target with no anchored summary -> None
- several summaries for one target -> the shared one first, then the lowest path
- only the first result is returned (LIMIT 1)

## Terms

symbol summary, file summary, anchored summary, summary lookup, one per symbol, shared note first

## Connections

- **Location:** `crates/myrn-engine/src/distill/dedupe.rs:99-109`
- **Signature:** `pub fn find_summary(conn: &Connection, target: &str) -> Result<Option<Existing>>`
- **In:** [[file-crates-myrn-engine-src-distill-dedupe-rs]]
- **Calls:** [[dedupe-load]]
- **Called by:** [[engine-record]]
