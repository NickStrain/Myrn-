---
id: dedupe-merge
type: summary
status: unverified
title: dedupe::merge
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/dedupe.rs#distill::dedupe::merge
anchors:
- sym:crates/myrn-engine/src/distill/dedupe.rs#distill::dedupe::merge@bf610c7d3e8a0862
---
# dedupe::merge

Mutates old in place: appends evidence and links not already present, updates an existing anchor's hash (even to None) or appends a new anchor by target, and sets signature only if old has none. With replace_body it overwrites body and title (title only if new has one); otherwise it appends new's trimmed body unless its whitespace-normalized text is already a substring of old's body. source is overwritten whenever new has one.

## Connections

- **Location:** `crates/myrn-engine/src/distill/dedupe.rs:124-160`
- **Signature:** `pub fn merge(old: &mut Note, new: &Note, replace_body: bool)`
- **In:** [[file-crates-myrn-engine-src-distill-dedupe-rs]]
- **Called by:** [[engine-record]]
- **Tests:** `merge_combines_and_refreshes_anchors`
