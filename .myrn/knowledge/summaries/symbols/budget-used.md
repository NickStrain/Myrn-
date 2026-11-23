---
id: budget-used
type: summary
status: unverified
title: Budget::used
source: myrn summarize
about: sym:crates/myrn-engine/src/pack/budget.rs#pack::budget::Budget::used
anchors:
- sym:crates/myrn-engine/src/pack/budget.rs#pack::budget::Budget::used@35624f14f76649a9
map_version: '2'
---
# Budget::used

Returns the total token count of all texts accepted by push so far.

## Behaviour

- fresh budget -> 0
- a rejected push leaves the result unchanged

## Terms

consumed, spent, current usage, tokens consumed, running total, accepted size

## Connections

- **Location:** `crates/myrn-engine/src/pack/budget.rs:28-30`
- **Signature:** `pub fn used(&self) -> usize`
- **In:** [[file-crates-myrn-engine-src-pack-budget-rs]]
