---
id: file-crates-myrn-engine-src-distill-dedupe-rs
type: summary
status: unverified
title: crates/myrn-engine/src/distill/dedupe.rs
source: myrn summarize
about: file:crates/myrn-engine/src/distill/dedupe.rs
anchors:
- file:crates/myrn-engine/src/distill/dedupe.rs@d67b1cd55ac28ff0
---
# crates/myrn-engine/src/distill/dedupe.rs

Decides whether a new bug, symbol summary or other note duplicates one already stored, and folds the new content into that note instead of inserting a second copy. Main parts: the Existing record, the lookups find_bug, find_summary and find_id, the helpers bug_symbols and parse_signature, and merge, which combines two notes.

## Connections

- **Location:** `crates/myrn-engine/src/distill/dedupe.rs`
- **In:** [[dir-crates-myrn-engine-src-distill]]
- **Defines:** [[dedupe-existing]], [[dedupe-load]], [[dedupe-parse-signature]], [[dedupe-bug-symbols]], [[dedupe-find-bug]], [[dedupe-find-summary]], [[dedupe-find-id]], [[dedupe-merge]]
- **Imports:** [[file-crates-myrn-engine-src-distill-signature-rs]], [[file-crates-myrn-store-src-notes-mod-rs]]
- **Imported by:** [[file-crates-myrn-engine-src-distill-mod-rs]], [[file-crates-myrn-engine-src-distill-record-rs]], [[file-crates-myrn-engine-src-retrieve-recall-rs]], [[file-crates-myrn-engine-src-sharing-rs]], [[file-crates-myrn-engine-src-validity-states-rs]]
