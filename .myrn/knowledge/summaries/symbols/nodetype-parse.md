---
id: nodetype-parse
type: summary
status: unverified
title: NodeType::parse
source: myrn summarize
about: sym:crates/myrn-model/src/node.rs#node::NodeType::parse
anchors:
- sym:crates/myrn-model/src/node.rs#node::NodeType::parse@741130c179d5c4b5
map_version: '2'
---
# NodeType::parse

Finds the NodeType whose as_str name equals the input by scanning NodeType::ALL, returning None if none matches.

## Behaviour

- "root_cause" -> Some(RootCause)
- "RootCause", "Root_Cause" or "" -> None, because matching is case-sensitive and does not trim whitespace
- Linear scan over the 12 entries of ALL

## Terms

from string, string to type, lookup, case-sensitive match, deserialize

## Connections

- **Location:** `crates/myrn-model/src/node.rs:65-67`
- **Signature:** `pub fn parse(s: &str) -> Option<NodeType>`
- **In:** [[file-crates-myrn-model-src-node-rs]]
- **Tests:** `round_trip`
