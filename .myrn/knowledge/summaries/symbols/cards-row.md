---
id: cards-row
type: summary
status: unverified
title: cards::row
source: myrn summarize
about: sym:crates/myrn-engine/src/pack/cards.rs#pack::cards::row
anchors:
- sym:crates/myrn-engine/src/pack/cards.rs#pack::cards::row@bbd10e02d2101cbc
---
# cards::row

Appends one line to the output string, padding the label to LABEL (11) columns before the value. Write errors are discarded.

## Connections

- **Location:** `crates/myrn-engine/src/pack/cards.rs:97-99`
- **Signature:** `fn row(out: &mut String, label: &str, value: &str)`
- **In:** [[file-crates-myrn-engine-src-pack-cards-rs]]
- **Called by:** [[cards-list]], [[symbolcard-render]], [[symbolcard-render-compact]], [[filecard-render]], [[impactreport-render]]
