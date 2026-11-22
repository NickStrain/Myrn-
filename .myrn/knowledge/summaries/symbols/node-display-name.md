---
id: node-display-name
type: summary
status: unverified
title: Node::display_name
source: myrn summarize
about: sym:crates/myrn-store/src/graph.rs#graph::Node::display_name
anchors:
- sym:crates/myrn-store/src/graph.rs#graph::Node::display_name@68e85ed2bfd6151f
---
# Node::display_name

Returns qname, else title, else path, else id, as the first non-empty option.

## Connections

- **Location:** `crates/myrn-store/src/graph.rs:72-78`
- **Signature:** `pub fn display_name(&self) -> &str`
- **In:** [[file-crates-myrn-store-src-graph-rs]]
- **Called by:** [[link-new]], [[render-one-line]]
