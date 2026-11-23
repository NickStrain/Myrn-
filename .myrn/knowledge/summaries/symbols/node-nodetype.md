---
id: node-nodetype
type: summary
status: unverified
title: node::NodeType
source: myrn summarize
about: sym:crates/myrn-model/src/node.rs#node::NodeType
anchors:
- sym:crates/myrn-model/src/node.rs#node::NodeType@5424b6eaa310e49f
map_version: '2'
---
# node::NodeType

Enum of the 12 node kinds. The first five (File, Symbol, Module, Commit, Test) are derived; the last seven (Feature, Decision, Fact, Bug, RootCause, Workflow, Episode) are earned.

## Behaviour

- Variants are listed in ALL in declaration order
- Serializes as snake_case, so RootCause becomes "root_cause"
- Derives Copy, Eq, Hash, Serialize and Deserialize

## Terms

node kind, category, entity type, derived node, earned node

## Connections

- **Location:** `crates/myrn-model/src/node.rs:15-30`
- **Signature:** `pub enum NodeType`
- **In:** [[file-crates-myrn-model-src-node-rs]]
