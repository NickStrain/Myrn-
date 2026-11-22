---
id: budget-push
type: summary
status: unverified
title: Budget::push
source: myrn summarize
about: sym:crates/myrn-engine/src/pack/budget.rs#pack::budget::Budget::push
anchors:
- sym:crates/myrn-engine/src/pack/budget.rs#pack::budget::Budget::push@84ba842a31b20414
---
# Budget::push

Appends s to out and adds its tokens to used only if used plus those tokens stays within limit, returning true. Otherwise it returns false and changes nothing; the text is not truncated, and a later smaller string can still fit.

## Connections

- **Location:** `crates/myrn-engine/src/pack/budget.rs:33-41`
- **Signature:** `pub fn push(&mut self, s: &str) -> bool`
- **In:** [[file-crates-myrn-engine-src-pack-budget-rs]]
- **Calls:** [[budget-tokens]]
- **Called by:** [[engine-brief]], [[render-pack]], [[engine-recall-text]]
- **Tests:** `respects_limit`
