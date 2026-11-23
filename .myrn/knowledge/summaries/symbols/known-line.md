---
id: known-line
type: summary
status: unverified
title: Known::line
source: myrn summarize
about: sym:crates/myrn-engine/src/pack/cards.rs#pack::cards::Known::line
anchors:
- sym:crates/myrn-engine/src/pack/cards.rs#pack::cards::Known::line@75525aceb031ffd7
map_version: '2'
---
# Known::line

Formats one learned record as a single line: id, then status and from in parentheses, then title. Does not trim or truncate the title.

## Behaviour

- id A, status S, from F, title T -> "A (S; F): T"
- empty title -> line ends with a colon and a space
- returns a new String each call

## Terms

record line, one-line form, known entry text, formatted record

## Connections

- **Location:** `crates/myrn-engine/src/pack/cards.rs:61-66`
- **Signature:** `fn line(&self) -> String`
- **In:** [[file-crates-myrn-engine-src-pack-cards-rs]]
