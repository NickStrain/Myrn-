---
id: link-new
type: summary
status: unverified
title: Link::new
source: myrn summarize
about: sym:crates/myrn-engine/src/pack/cards.rs#pack::cards::Link::new
anchors:
- sym:crates/myrn-engine/src/pack/cards.rs#pack::cards::Link::new@8707e1336735540b
map_version: '2'
---
# Link::new

Builds a Link from a Node and a status, copying the node id and display name and computing location unless the node is a file. The status is passed through unchanged.

## Behaviour

- ntype "file" -> location is empty
- other ntype -> location from n.location()
- status None stays None

## Terms

constructor, node to link, display name copy, file location blank, build reference

## Connections

- **Location:** `crates/myrn-engine/src/pack/cards.rs:18-30`
- **Signature:** `pub fn new(n: &Node, status: Option<String>) -> Link`
- **In:** [[file-crates-myrn-engine-src-pack-cards-rs]]
- **Calls:** [[node-location]], [[node-display-name]]
- **Called by:** [[engine-pick]], [[engine-links]], [[engine-symbol-card]], [[engine-file-card]], [[engine-impact]]
