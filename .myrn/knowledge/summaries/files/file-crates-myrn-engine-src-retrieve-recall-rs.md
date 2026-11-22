---
id: file-crates-myrn-engine-src-retrieve-recall-rs
type: summary
status: unverified
title: crates/myrn-engine/src/retrieve/recall.rs
source: myrn summarize
about: file:crates/myrn-engine/src/retrieve/recall.rs
anchors:
- file:crates/myrn-engine/src/retrieve/recall.rs@f27677861e0c7a1c
---
# crates/myrn-engine/src/retrieve/recall.rs

Recall of known bugs for an error message: normalizes the error into a signature, matches it against recorded bug nodes, and renders the best matches with root causes, locations, fixing commits, and fix text into a token-budgeted quoted block. Main parts are RecallHit, Engine::recall (matching and graph walk), and Engine::recall_text (budgeted rendering).

## Connections

- **Location:** `crates/myrn-engine/src/retrieve/recall.rs`
- **In:** [[dir-crates-myrn-engine-src-retrieve]]
- **Defines:** [[recall-recallhit]], [[engine-recall]], [[engine-recall-text]], [[recall-clip]], [[recall-first-paragraph]]
- **Imports:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-engine-src-distill-dedupe-rs]], [[file-crates-myrn-engine-src-distill-signature-rs]], [[file-crates-myrn-engine-src-pack-budget-rs]], [[file-crates-myrn-store-src-graph-rs]]
- **Imported by:** [[file-crates-myrn-engine-src-retrieve-mod-rs]]
