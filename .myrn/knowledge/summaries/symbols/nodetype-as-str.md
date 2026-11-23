---
id: nodetype-as-str
type: summary
status: unverified
title: NodeType::as_str
source: myrn summarize
about: sym:crates/myrn-model/src/node.rs#node::NodeType::as_str
anchors:
- sym:crates/myrn-model/src/node.rs#node::NodeType::as_str@87b15ccc2e677f3b
map_version: '2'
---
# NodeType::as_str

Returns the fixed lowercase snake_case name of a node type as a static string.

## Behaviour

- RootCause -> "root_cause"
- Test -> "test"
- Exhaustive match with no fallback and no allocation

## Terms

string name, label, serialized name, snake_case, text form

## Connections

- **Location:** `crates/myrn-model/src/node.rs:48-63`
- **Signature:** `pub fn as_str(self) -> &'static str`
- **In:** [[file-crates-myrn-model-src-node-rs]]
