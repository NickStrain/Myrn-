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

Matches new bug, summary, and id-keyed notes against notes already in the store so they are updated instead of duplicated, and folds matched notes together with merge. It holds the SAME_BUG similarity threshold (0.75), the Existing wrapper, the lookup queries (find_bug, find_summary, find_id), bug_symbols, and the merge logic.

## Connections

- **Location:** `crates/myrn-engine/src/distill/dedupe.rs`
- **In:** [[dir-crates-myrn-engine-src-distill]]
- **Defines:** [[dedupe-existing]], [[dedupe-load]], [[dedupe-parse-signature]], [[dedupe-bug-symbols]], [[dedupe-find-bug]], [[dedupe-find-summary]], [[dedupe-find-id]], [[dedupe-merge]]
- **Imports:** [[file-crates-myrn-engine-src-distill-signature-rs]], [[file-crates-myrn-store-src-notes-mod-rs]]
- **Imported by:** [[file-crates-myrn-engine-src-distill-mod-rs]], [[file-crates-myrn-engine-src-distill-record-rs]], [[file-crates-myrn-engine-src-retrieve-recall-rs]]
