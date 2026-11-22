---
id: nodetype-parse
type: summary
status: unverified
title: NodeType::parse
source: myrn summarize
about: sym:crates/myrn-model/src/node.rs#node::NodeType::parse
anchors:
- sym:crates/myrn-model/src/node.rs#node::NodeType::parse@741130c179d5c4b5
---
# NodeType::parse

Looks up a NodeType from its string name by scanning ALL, returning None for unknown strings; case-sensitive.

## Connections

- **Location:** `crates/myrn-model/src/node.rs:65-67`
- **Signature:** `pub fn parse(s: &str) -> Option<NodeType>`
- **In:** [[file-crates-myrn-model-src-node-rs]]
- **Tests:** `round_trip`
