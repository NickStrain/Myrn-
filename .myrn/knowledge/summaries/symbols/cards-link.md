---
id: cards-link
type: summary
status: unverified
title: cards::Link
source: myrn summarize
about: sym:crates/myrn-engine/src/pack/cards.rs#pack::cards::Link
anchors:
- sym:crates/myrn-engine/src/pack/cards.rs#pack::cards::Link@133b1dcd171d1524
map_version: '2'
---
# cards::Link

Holds one related node as id, display name, file location and optional call status. Built from a graph Node by Link::new.

## Behaviour

- file node -> location is an empty string
- non-file node -> location taken from n.location()
- status is stored as given: Some("inferred") or None for non-call relations

## Terms

reference, neighbour, call edge status, resolved or inferred, relation entry

## Connections

- **Location:** `crates/myrn-engine/src/pack/cards.rs:9-15`
- **Signature:** `pub struct Link`
- **In:** [[file-crates-myrn-engine-src-pack-cards-rs]]
