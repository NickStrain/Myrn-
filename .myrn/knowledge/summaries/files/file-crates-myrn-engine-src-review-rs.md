---
id: file-crates-myrn-engine-src-review-rs
type: summary
status: unverified
title: crates/myrn-engine/src/review.rs
source: myrn summarize
about: file:crates/myrn-engine/src/review.rs
anchors:
- file:crates/myrn-engine/src/review.rs@71dae9aa03961467
---
# crates/myrn-engine/src/review.rs

Gate 2 hand review of learned records: exports notes with their evidence and linked code excerpts, samples them into a Markdown checklist that a person marks correct, vague or wrong, and reads the marks back into precision figures. Main parts are the export (Engine::review_items, Engine::excerpt), sample, checklist, score, and the tests module.

## Connections

- **Location:** `crates/myrn-engine/src/review.rs`
- **In:** [[dir-crates-myrn-engine-src]]
- **Defines:** [[review-reviewitem]], [[review-coderef]], [[engine-review-items]], [[engine-excerpt]], [[review-sample]], [[review-checklist]], [[review-score]], [[review-score-review]], [[tests-item]]
- **Imports:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-store-src-graph-rs]]
- **Imported by:** [[file-crates-myrn-engine-src-lib-rs]], [[file-crates-myrn-src-cmd-debug-rs]]
