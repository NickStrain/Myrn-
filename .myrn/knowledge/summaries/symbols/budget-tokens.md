---
id: budget-tokens
type: summary
status: unverified
title: budget::tokens
source: myrn summarize
about: sym:crates/myrn-engine/src/pack/budget.rs#pack::budget::tokens
anchors:
- sym:crates/myrn-engine/src/pack/budget.rs#pack::budget::tokens@9a11f2b97802b5e5
map_version: '2'
---
# budget::tokens

Estimates the token count of a string as its Unicode character count divided by four, rounded up.

## Behaviour

- empty string -> 0
- abcd (4 chars) -> 1; abcde (5 chars) -> 2
- counts chars, not bytes: 4 multibyte chars -> 1

## Terms

size estimate, character count, approximate length, ceiling division, rounds up, chars per token

## Connections

- **Location:** `crates/myrn-engine/src/pack/budget.rs:4-6`
- **Signature:** `pub fn tokens(s: &str) -> usize`
- **In:** [[file-crates-myrn-engine-src-pack-budget-rs]]
- **Called by:** [[engine-context-excluding]], [[engine-brief]], [[budget-push]]
