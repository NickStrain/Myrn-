---
id: budget-budget
type: summary
status: unverified
title: budget::Budget
source: myrn summarize
about: sym:crates/myrn-engine/src/pack/budget.rs#pack::budget::Budget
anchors:
- sym:crates/myrn-engine/src/pack/budget.rs#pack::budget::Budget@b1c507c432f25168
map_version: '2'
---
# budget::Budget

Holds a token limit, a running count of tokens used, and the accumulated output string. Only out is public; limit and used are private and change only through push.

## Behaviour

- used never exceeds limit, because push refuses any text that would overflow it
- out is the only way to read the accumulated text

## Terms

size cap, allowance, context limit, bounded buffer, capacity, text accumulator

## Connections

- **Location:** `crates/myrn-engine/src/pack/budget.rs:9-13`
- **Signature:** `pub struct Budget`
- **In:** [[file-crates-myrn-engine-src-pack-budget-rs]]
