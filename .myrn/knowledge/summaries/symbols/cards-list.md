---
id: cards-list
type: summary
status: unverified
title: cards::list
source: myrn summarize
about: sym:crates/myrn-engine/src/pack/cards.rs#pack::cards::list
anchors:
- sym:crates/myrn-engine/src/pack/cards.rs#pack::cards::list@5468ea6ab613ac81
map_version: '2'
---
# cards::list

Writes a labelled list of Links to out, showing at most limit items with an aligned name column, a tilde for inferred items, and a trailing count of hidden items. The label appears only on the first row.

## Behaviour

- empty items -> a single row reading "none found"
- more than limit items -> a final row reading " ... N more"
- name column width is the longest of the first limit names, capped at 48 characters; longer names overflow rather than being cut

## Terms

aligned list, truncated list, name column, inferred marker, hidden count

## Connections

- **Location:** `crates/myrn-engine/src/pack/cards.rs:106-130`
- **Signature:** `fn list(out: &mut String, label: &str, items: &[Link], limit: usize)`
- **In:** [[file-crates-myrn-engine-src-pack-cards-rs]]
- **Calls:** [[cards-row]]
- **Called by:** [[symbolcard-render]], [[symbolcard-render-compact]], [[filecard-render]], [[impactreport-render]]
