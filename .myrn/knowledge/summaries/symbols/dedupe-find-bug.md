---
id: dedupe-find-bug
type: summary
status: unverified
title: dedupe::find_bug
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/dedupe.rs#distill::dedupe::find_bug
anchors:
- sym:crates/myrn-engine/src/distill/dedupe.rs#distill::dedupe::find_bug@f0f09463ea9dce97
---
# dedupe::find_bug

Scans all bug notes (shared first, then by path), skips those without a signature, keeps those with similarity >= 0.75, and drops any whose non-empty symbol set is disjoint from the caller's non-empty symbols; returns the highest-similarity match, with ties going to the earlier row. Read-only.

## Connections

- **Location:** `crates/myrn-engine/src/distill/dedupe.rs:68-96`
- **Signature:** `pub fn find_bug( conn: &Connection, sig: &Signature, symbols: &HashSet<String>, ) -> Result<Option<Existing>>`
- **In:** [[file-crates-myrn-engine-src-distill-dedupe-rs]]
- **Calls:** [[dedupe-load]], [[dedupe-bug-symbols]], [[signature-similarity]]
- **Called by:** [[engine-record]]
