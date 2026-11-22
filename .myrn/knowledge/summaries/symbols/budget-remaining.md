---
id: budget-remaining
type: summary
status: unverified
title: Budget::remaining
source: myrn summarize
about: sym:crates/myrn-engine/src/pack/budget.rs#pack::budget::Budget::remaining
anchors:
- sym:crates/myrn-engine/src/pack/budget.rs#pack::budget::Budget::remaining@7509c600fb698ab8
---
# Budget::remaining

Returns limit minus used, saturating at 0 so it never underflows.

## Connections

- **Location:** `crates/myrn-engine/src/pack/budget.rs:24-26`
- **Signature:** `pub fn remaining(&self) -> usize`
- **In:** [[file-crates-myrn-engine-src-pack-budget-rs]]
- **Tests:** `respects_limit`
