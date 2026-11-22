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

Approximate token counting and a token-limited text accumulator for pack building. tokens() estimates a string's token count, and Budget collects text into a public out String while enforcing a limit. The file also has a unit test for the limit behavior.

## Connections

- **Location:** `crates/myrn-engine/src/pack/budget.rs`
- **In:** [[dir-crates-myrn-engine-src-pack]]
- **Defines:** [[budget-tokens]], [[budget-budget]], [[budget-new]], [[budget-remaining]], [[budget-used]], [[budget-push]]
- **Imported by:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-engine-src-pack-mod-rs]], [[file-crates-myrn-engine-src-pack-render-rs]], [[file-crates-myrn-engine-src-retrieve-recall-rs]]
