---
id: budget-remaining
type: summary
status: unverified
title: Budget::remaining
source: myrn summarize
about: sym:crates/myrn-engine/src/pack/budget.rs#pack::budget::Budget::remaining
anchors:
- sym:crates/myrn-engine/src/pack/budget.rs#pack::budget::Budget::remaining@7509c600fb698ab8
map_version: '2'
---
# Budget::remaining

Returns the tokens still available, computed with saturating subtraction so it never underflows.

## Behaviour

- limit 3, used 2 -> 1
- limit 3, used 3 -> 0
- result is never negative

## Terms

left over, headroom, unused allowance, capacity left, tokens available, saturating subtraction

## Connections

- **Location:** `crates/myrn-engine/src/pack/budget.rs:24-26`
- **Signature:** `pub fn remaining(&self) -> usize`
- **In:** [[file-crates-myrn-engine-src-pack-budget-rs]]
- **Tests:** `respects_limit`
