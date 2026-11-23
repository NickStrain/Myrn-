---
id: cards-row
type: summary
status: unverified
title: cards::row
source: myrn summarize
about: sym:crates/myrn-engine/src/pack/cards.rs#pack::cards::row
anchors:
- sym:crates/myrn-engine/src/pack/cards.rs#pack::cards::row@bbd10e02d2101cbc
map_version: '2'
---
# cards::row

Writes one line to out: the label padded to 11 columns, then the value, then a newline. Any write error is discarded.

## Behaviour

- label longer than 11 characters -> no padding, label runs into value
- empty label -> value starts at column 12, used for continuation lines
- value is written as given, with no trimming

## Terms

line writer, label column, aligned row, continuation line, formatting helper

## Connections

- **Location:** `crates/myrn-engine/src/pack/cards.rs:102-104`
- **Signature:** `fn row(out: &mut String, label: &str, value: &str)`
- **In:** [[file-crates-myrn-engine-src-pack-cards-rs]]
- **Called by:** [[cards-list]], [[symbolcard-render]], [[symbolcard-render-compact]], [[filecard-render]], [[impactreport-render]]
