---
id: budget-new
type: summary
status: unverified
title: Budget::new
source: myrn summarize
about: sym:crates/myrn-engine/src/pack/budget.rs#pack::budget::Budget::new
anchors:
- sym:crates/myrn-engine/src/pack/budget.rs#pack::budget::Budget::new@5a244d3faab05cf4
map_version: '2'
---
# Budget::new

Creates an empty budget with the given token limit, zero tokens used, and an empty out string.

## Behaviour

- limit 0 -> remaining() is 0 and every non-empty push returns false
- fresh budget -> used() is 0 and out is empty

## Terms

constructor, initialize, empty buffer, zeroed counter, create, set up limit

## Connections

- **Location:** `crates/myrn-engine/src/pack/budget.rs:16-22`
- **Signature:** `pub fn new(limit: usize) -> Budget`
- **In:** [[file-crates-myrn-engine-src-pack-budget-rs]]
- **Called by:** [[engine-brief]], [[render-pack]], [[engine-recall-text]], [[engine-suspect-notice]]
- **Tests:** `respects_limit`
