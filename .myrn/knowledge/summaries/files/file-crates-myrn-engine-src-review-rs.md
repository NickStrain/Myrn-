---
id: file-crates-myrn-engine-src-review-rs
type: summary
status: unverified
title: crates/myrn-engine/src/review.rs
source: myrn summarize
about: file:crates/myrn-engine/src/review.rs
anchors:
- file:crates/myrn-engine/src/review.rs@b4ae3efae97a9792
---
# crates/myrn-engine/src/review.rs

Gate 2 hand review of learned records. It exports each note with its evidence and linked code excerpts, samples them into a Markdown checklist that a person marks correct, vague, or wrong, and parses those marks back into precision figures. Main parts: Engine::review_items (export), Engine::excerpt (code reader), sample, checklist (renderer), and score (mark parser).

## Connections

- **Location:** `crates/myrn-engine/src/review.rs`
- **In:** [[dir-crates-myrn-engine-src]]
- **Defines:** [[review-reviewitem]], [[review-coderef]], [[engine-review-items]], [[engine-excerpt]], [[review-sample]], [[review-checklist]], [[review-score]], [[review-score-review]], [[tests-item]]
- **Imports:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-store-src-graph-rs]]
- **Imported by:** [[file-crates-myrn-engine-src-lib-rs]], [[file-crates-myrn-src-cmd-debug-rs]]
