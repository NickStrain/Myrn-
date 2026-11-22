---
id: dedupe-bug-symbols
type: summary
status: unverified
title: dedupe::bug_symbols
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/dedupe.rs#distill::dedupe::bug_symbols
anchors:
- sym:crates/myrn-engine/src/distill/dedupe.rs#distill::dedupe::bug_symbols@cb44416eb06a1f82
---
# dedupe::bug_symbols

Returns the set of non-null symbol_ids from anchors on the bug note itself and on notes it points to via caused_by edges, so root-cause code counts toward matching.

## Connections

- **Location:** `crates/myrn-engine/src/distill/dedupe.rs:58-65`
- **Signature:** `pub fn bug_symbols(conn: &Connection, bug_id: &str) -> Result<HashSet<String>>`
- **In:** [[file-crates-myrn-engine-src-distill-dedupe-rs]]
- **Called by:** [[dedupe-find-bug]]
