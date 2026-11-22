---
id: cards-kind-of
type: summary
status: unverified
title: cards::kind_of
source: myrn summarize
about: sym:crates/myrn-engine/src/pack/cards.rs#pack::cards::kind_of
anchors:
- sym:crates/myrn-engine/src/pack/cards.rs#pack::cards::kind_of@4a1cd4462379bd54
---
# cards::kind_of

Returns the node's kind if set, otherwise its ntype. The result is used as the card header label and borrows from the node.

## Connections

- **Location:** `crates/myrn-engine/src/pack/cards.rs:127-129`
- **Signature:** `fn kind_of(n: &Node) -> &str`
- **In:** [[file-crates-myrn-engine-src-pack-cards-rs]]
- **Called by:** [[symbolcard-render]], [[symbolcard-render-compact]]
