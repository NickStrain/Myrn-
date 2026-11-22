---
id: budget-tokens
type: summary
status: unverified
title: budget::tokens
source: myrn summarize
about: sym:crates/myrn-engine/src/pack/budget.rs#pack::budget::tokens
anchors:
- sym:crates/myrn-engine/src/pack/budget.rs#pack::budget::tokens@9a11f2b97802b5e5
---
# budget::tokens

Returns the approximate token count of s as its character count divided by 4, rounded up. It counts chars, not bytes, so an empty string is 0 and 1 to 4 chars is 1.

## Connections

- **Location:** `crates/myrn-engine/src/pack/budget.rs:4-6`
- **Signature:** `pub fn tokens(s: &str) -> usize`
- **In:** [[file-crates-myrn-engine-src-pack-budget-rs]]
- **Called by:** [[engine-context-excluding]], [[engine-brief]], [[budget-push]]
