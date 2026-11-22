---
id: nodetype-origin
type: summary
status: unverified
title: NodeType::origin
source: myrn summarize
about: sym:crates/myrn-model/src/node.rs#node::NodeType::origin
anchors:
- sym:crates/myrn-model/src/node.rs#node::NodeType::origin@d4ff310a34a042d4
---
# NodeType::origin

Classifies a variant as Origin::Derived for the five code-derived kinds and Origin::Earned for everything else via a wildcard arm, so new variants default to Earned.

## Connections

- **Location:** `crates/myrn-model/src/node.rs:69-78`
- **Signature:** `pub fn origin(self) -> Origin`
- **In:** [[file-crates-myrn-model-src-node-rs]]
- **Tests:** `round_trip`
