---
id: dedupe-bug-symbols
type: summary
status: unverified
title: dedupe::bug_symbols
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/dedupe.rs#distill::dedupe::bug_symbols
anchors:
- sym:crates/myrn-engine/src/distill/dedupe.rs#distill::dedupe::bug_symbols@cb44416eb06a1f82
map_version: '2'
---
# dedupe::bug_symbols

Returns the set of symbol ids a bug touches: its own anchored symbols plus those of notes it points to with caused_by edges.

## Behaviour

- bug with no anchors and no caused_by edges -> empty set
- anchors with NULL symbol_id are excluded
- only direct caused_by targets are followed, not transitive causes

## Terms

touched code, affected code, root cause, caused_by, anchored code, involved code

## Connections

- **Location:** `crates/myrn-engine/src/distill/dedupe.rs:58-65`
- **Signature:** `pub fn bug_symbols(conn: &Connection, bug_id: &str) -> Result<HashSet<String>>`
- **In:** [[file-crates-myrn-engine-src-distill-dedupe-rs]]
- **Called by:** [[dedupe-find-bug]]
