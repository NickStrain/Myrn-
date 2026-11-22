---
id: link-new
type: summary
status: unverified
title: Link::new
source: myrn summarize
about: sym:crates/myrn-engine/src/pack/cards.rs#pack::cards::Link::new
anchors:
- sym:crates/myrn-engine/src/pack/cards.rs#pack::cards::Link::new@8707e1336735540b
---
# Link::new

Builds a Link from a Node and an optional edge status. Location is empty for nodes of type `file` and otherwise comes from Node::location().

## Connections

- **Location:** `crates/myrn-engine/src/pack/cards.rs:18-30`
- **Signature:** `pub fn new(n: &Node, status: Option<String>) -> Link`
- **In:** [[file-crates-myrn-engine-src-pack-cards-rs]]
- **Calls:** [[node-location]], [[node-display-name]]
- **Called by:** [[engine-pick]], [[engine-links]], [[engine-symbol-card]], [[engine-file-card]], [[engine-impact]]
