---
id: cards-known
type: summary
status: unverified
title: cards::Known
source: myrn summarize
about: sym:crates/myrn-engine/src/pack/cards.rs#pack::cards::Known
anchors:
- sym:crates/myrn-engine/src/pack/cards.rs#pack::cards::Known@b69659a7a29b796a
map_version: '2'
---
# cards::Known

A learned record attached to a symbol card, with id, type, status, title and a from field saying whether it is shared or private, who wrote it and when. The ntype field is serialised as "type".

## Behaviour

- ntype serialises to the JSON key "type"
- from is free text from the source record, shown in parentheses by line()
- status is a plain string, not an enum

## Terms

learned record, memory entry, decision, root cause, bug note

## Connections

- **Location:** `crates/myrn-engine/src/pack/cards.rs:50-58`
- **Signature:** `pub struct Known`
- **In:** [[file-crates-myrn-engine-src-pack-cards-rs]]
