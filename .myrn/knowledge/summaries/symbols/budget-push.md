---
id: budget-push
type: summary
status: unverified
title: Budget::push
source: myrn summarize
about: sym:crates/myrn-engine/src/pack/budget.rs#pack::budget::Budget::push
anchors:
- sym:crates/myrn-engine/src/pack/budget.rs#pack::budget::Budget::push@84ba842a31b20414
map_version: '2'
---
# Budget::push

Appends text to out and adds its token count to used only if it fits in the remaining allowance; returns true if appended, false otherwise with no change to out or used.

## Behaviour

- remaining 2, 8-char text (2 tokens) -> true, remaining becomes 0 (exact fit accepted)
- text that does not fit -> false, out and used unchanged
- empty string -> true, appends nothing; a later smaller text can still fit after a larger one was rejected

## Terms

append, add text, fit check, conditional append, reject oversize, try add, overflow guard

## Connections

- **Location:** `crates/myrn-engine/src/pack/budget.rs:33-41`
- **Signature:** `pub fn push(&mut self, s: &str) -> bool`
- **In:** [[file-crates-myrn-engine-src-pack-budget-rs]]
- **Calls:** [[budget-tokens]]
- **Called by:** [[engine-brief]], [[render-pack]], [[engine-recall-text]], [[engine-suspect-notice]]
- **Tests:** `respects_limit`
