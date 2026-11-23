---
id: cards-kind-of
type: summary
status: unverified
title: cards::kind_of
source: myrn summarize
about: sym:crates/myrn-engine/src/pack/cards.rs#pack::cards::kind_of
anchors:
- sym:crates/myrn-engine/src/pack/cards.rs#pack::cards::kind_of@4a1cd4462379bd54
map_version: '2'
---
# cards::kind_of

Returns the kind of a node, falling back to its node type when no kind is set. Returns a borrowed string without allocating.

## Behaviour

- kind Some("K") -> "K"
- kind None and ntype "file" -> "file"
- kind Some("") -> empty string, since only None triggers the fallback

## Terms

node kind, node type, header label, type fallback

## Connections

- **Location:** `crates/myrn-engine/src/pack/cards.rs:132-134`
- **Signature:** `fn kind_of(n: &Node) -> &str`
- **In:** [[file-crates-myrn-engine-src-pack-cards-rs]]
- **Called by:** [[symbolcard-render]], [[symbolcard-render-compact]]
