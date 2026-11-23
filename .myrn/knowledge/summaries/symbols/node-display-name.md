---
id: node-display-name
type: summary
status: unverified
title: Node::display_name
source: myrn summarize
about: sym:crates/myrn-store/src/graph.rs#graph::Node::display_name
anchors:
- sym:crates/myrn-store/src/graph.rs#graph::Node::display_name@68e85ed2bfd6151f
map_version: '2'
---
# Node::display_name

Returns the best label for the node, trying qname, then title, then path, then id.

## Behaviour

- qname Some -> qname, even if title is also set
- qname None and title Some -> title
- qname, title and path all None -> id

## Terms

label, name, friendly name, caption, fallback name, best name

## Connections

- **Location:** `crates/myrn-store/src/graph.rs:72-78`
- **Signature:** `pub fn display_name(&self) -> &str`
- **In:** [[file-crates-myrn-store-src-graph-rs]]
- **Called by:** [[link-new]], [[render-one-line]], [[viz-gnode-viz]]
