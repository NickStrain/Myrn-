---
id: file-crates-myrn-engine-src-pack-budget-rs
type: summary
status: unverified
title: crates/myrn-engine/src/pack/budget.rs
source: myrn summarize
about: file:crates/myrn-engine/src/pack/budget.rs
anchors:
- file:crates/myrn-engine/src/pack/budget.rs@845203274bb397d1
---
# crates/myrn-engine/src/pack/budget.rs

Token budget helper for packs. tokens() estimates a string's size as its character count divided by four, rounded up, and Budget is a bounded text accumulator that rejects any push that would exceed its token limit. The file also has one unit test for the limit.

## Connections

- **Location:** `crates/myrn-engine/src/pack/budget.rs`
- **In:** [[dir-crates-myrn-engine-src-pack]]
- **Defines:** [[budget-tokens]], [[budget-budget]], [[budget-new]], [[budget-remaining]], [[budget-used]], [[budget-push]]
- **Imported by:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-engine-src-pack-mod-rs]], [[file-crates-myrn-engine-src-pack-render-rs]], [[file-crates-myrn-engine-src-retrieve-recall-rs]], [[file-crates-myrn-engine-src-validity-anchors-rs]]
