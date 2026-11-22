---
id: budget-new
type: summary
status: unverified
title: Budget::new
source: myrn summarize
about: sym:crates/myrn-engine/src/pack/budget.rs#pack::budget::Budget::new
anchors:
- sym:crates/myrn-engine/src/pack/budget.rs#pack::budget::Budget::new@5a244d3faab05cf4
---
# Budget::new

Creates an empty Budget with the given token limit, zero used tokens, and an empty out string.

## Connections

- **Location:** `crates/myrn-engine/src/pack/budget.rs:16-22`
- **Signature:** `pub fn new(limit: usize) -> Budget`
- **In:** [[file-crates-myrn-engine-src-pack-budget-rs]]
- **Called by:** [[engine-brief]], [[render-pack]], [[engine-recall-text]]
- **Tests:** `respects_limit`
