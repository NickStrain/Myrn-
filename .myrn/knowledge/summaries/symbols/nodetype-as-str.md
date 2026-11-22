---
id: nodetype-as-str
type: summary
status: unverified
title: NodeType::as_str
source: myrn summarize
about: sym:crates/myrn-model/src/node.rs#node::NodeType::as_str
anchors:
- sym:crates/myrn-model/src/node.rs#node::NodeType::as_str@87b15ccc2e677f3b
---
# NodeType::as_str

Returns the snake_case string name for a variant (e.g. RootCause to "root_cause") as a static str; must be kept in sync with the enum when variants are added.

## Connections

- **Location:** `crates/myrn-model/src/node.rs:48-63`
- **Signature:** `pub fn as_str(self) -> &'static str`
- **In:** [[file-crates-myrn-model-src-node-rs]]
