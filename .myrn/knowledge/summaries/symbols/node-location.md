---
id: node-location
type: summary
status: unverified
title: Node::location
source: myrn summarize
about: sym:crates/myrn-store/src/graph.rs#graph::Node::location
anchors:
- sym:crates/myrn-store/src/graph.rs#graph::Node::location@31f211a751fa3749
---
# Node::location

Returns path:start-end when the lines differ, path:start when they are equal or end is missing, and just path when start is missing.

## Connections

- **Location:** `crates/myrn-store/src/graph.rs:62-69`
- **Signature:** `pub fn location(&self) -> String`
- **In:** [[file-crates-myrn-store-src-graph-rs]]
- **Called by:** [[link-new]], [[symbolcard-render]], [[symbolcard-render-compact]], [[render-one-line]], [[engine-recall]], [[engine-review-items]], [[engine-map-body]]
