---
id: file-crates-myrn-engine-src-retrieve-recall-rs
type: summary
status: unverified
title: crates/myrn-engine/src/retrieve/recall.rs
source: myrn summarize
about: file:crates/myrn-engine/src/retrieve/recall.rs
anchors:
- file:crates/myrn-engine/src/retrieve/recall.rs@04cc9a725be4acec
---
# crates/myrn-engine/src/retrieve/recall.rs

Known-bug recall: matches an error text against stored bug signatures (similarity at least 0.6) and returns each hit with root causes, fixing commits and provenance, or renders them as a token-budgeted quoted block. Main parts are the RecallHit record, Engine::recall (graph query and ranking), Engine::recall_text (budgeted text output), and the clip and first_paragraph helpers.

## Connections

- **Location:** `crates/myrn-engine/src/retrieve/recall.rs`
- **In:** [[dir-crates-myrn-engine-src-retrieve]]
- **Defines:** [[recall-recallhit]], [[engine-recall]], [[engine-recall-text]], [[recall-clip]], [[recall-first-paragraph]]
- **Imports:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-engine-src-distill-dedupe-rs]], [[file-crates-myrn-engine-src-distill-signature-rs]], [[file-crates-myrn-engine-src-pack-budget-rs]], [[file-crates-myrn-engine-src-pack-render-rs]], [[file-crates-myrn-store-src-graph-rs]]
- **Imported by:** [[file-crates-myrn-engine-src-retrieve-mod-rs]]
