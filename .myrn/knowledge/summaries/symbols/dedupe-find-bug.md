---
id: dedupe-find-bug
type: summary
status: unverified
title: dedupe::find_bug
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/dedupe.rs#distill::dedupe::find_bug
anchors:
- sym:crates/myrn-engine/src/distill/dedupe.rs#distill::dedupe::find_bug@f0f09463ea9dce97
map_version: '2'
---
# dedupe::find_bug

Finds the stored bug notes whose signature is at least SAME_BUG (0.75) similar to the given signature and whose code overlaps the given symbols, and returns the most similar one.

## Behaviour

- similarity 0.75 -> accepted; 0.74 -> rejected
- either side with no symbols -> code check skipped and signature alone decides
- both sides have symbols but none in common -> rejected even with an identical signature

## Terms

duplicate bug, same error, signature match, code overlap, fuzzy match, best match

## Connections

- **Location:** `crates/myrn-engine/src/distill/dedupe.rs:68-96`
- **Signature:** `pub fn find_bug( conn: &Connection, sig: &Signature, symbols: &HashSet<String>, ) -> Result<Option<Existing>>`
- **In:** [[file-crates-myrn-engine-src-distill-dedupe-rs]]
- **Calls:** [[dedupe-load]], [[dedupe-bug-symbols]], [[signature-similarity]]
- **Called by:** [[engine-record]]
