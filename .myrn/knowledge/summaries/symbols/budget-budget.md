---
id: budget-budget
type: summary
status: unverified
title: budget::Budget
source: myrn summarize
about: sym:crates/myrn-engine/src/pack/budget.rs#pack::budget::Budget
anchors:
- sym:crates/myrn-engine/src/pack/budget.rs#pack::budget::Budget@b1c507c432f25168
---
# budget::Budget

Holds a private limit and used count plus a public out String that collects accepted text. Gotcha: out is pub, so writing to it directly bypasses the used accounting.

## Connections

- **Location:** `crates/myrn-engine/src/pack/budget.rs:9-13`
- **Signature:** `pub struct Budget`
- **In:** [[file-crates-myrn-engine-src-pack-budget-rs]]
