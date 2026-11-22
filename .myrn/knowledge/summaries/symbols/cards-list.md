---
id: cards-list
type: summary
status: unverified
title: cards::list
source: myrn summarize
about: sym:crates/myrn-engine/src/pack/cards.rs#pack::cards::list
anchors:
- sym:crates/myrn-engine/src/pack/cards.rs#pack::cards::list@5468ea6ab613ac81
---
# cards::list

Appends a labeled list of up to `limit` items. Each name is padded to the longest of those items, capped at 48 characters, and inferred links are marked with `~`. Prints 'none found' when the list is empty and '… N more' when items were cut off.

## Connections

- **Location:** `crates/myrn-engine/src/pack/cards.rs:101-125`
- **Signature:** `fn list(out: &mut String, label: &str, items: &[Link], limit: usize)`
- **In:** [[file-crates-myrn-engine-src-pack-cards-rs]]
- **Calls:** [[cards-row]]
- **Called by:** [[symbolcard-render]], [[symbolcard-render-compact]], [[filecard-render]], [[impactreport-render]]
