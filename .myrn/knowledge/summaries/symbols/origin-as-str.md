---
id: origin-as-str
type: summary
status: unverified
title: Origin::as_str
source: myrn summarize
about: sym:crates/myrn-model/src/node.rs#node::Origin::as_str
anchors:
- sym:crates/myrn-model/src/node.rs#node::Origin::as_str@a522257c3f0744f2
map_version: '2'
---
# Origin::as_str

Returns "derived" or "earned" as a static string for the origin.

## Behaviour

- Derived -> "derived"
- Earned -> "earned"
- Exhaustive match with no fallback

## Terms

label, string name, text form, serialized value

## Connections

- **Location:** `crates/myrn-model/src/node.rs:82-87`
- **Signature:** `pub fn as_str(self) -> &'static str`
- **In:** [[file-crates-myrn-model-src-node-rs]]
- **Tests:** `notes_become_nodes_edges_anchors_and_summaries`
