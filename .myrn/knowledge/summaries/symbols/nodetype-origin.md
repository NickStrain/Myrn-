---
id: nodetype-origin
type: summary
status: unverified
title: NodeType::origin
source: myrn summarize
about: sym:crates/myrn-model/src/node.rs#node::NodeType::origin
anchors:
- sym:crates/myrn-model/src/node.rs#node::NodeType::origin@d4ff310a34a042d4
map_version: '2'
---
# NodeType::origin

Classifies a node type as Derived (File, Symbol, Module, Commit, Test) or Earned (every other variant), using a match with a wildcard arm.

## Behaviour

- Test -> Derived
- Bug -> Earned
- Any variant missing from the five-item derived list falls through the wildcard to Earned, so a new variant is earned unless this match is updated

## Terms

provenance, derived or earned, classification, which origin, rebuildable

## Connections

- **Location:** `crates/myrn-model/src/node.rs:69-78`
- **Signature:** `pub fn origin(self) -> Origin`
- **In:** [[file-crates-myrn-model-src-node-rs]]
- **Tests:** `round_trip`
