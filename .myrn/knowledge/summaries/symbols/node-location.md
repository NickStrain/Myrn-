---
id: node-location
type: summary
status: unverified
title: Node::location
source: myrn summarize
about: sym:crates/myrn-store/src/graph.rs#graph::Node::location
anchors:
- sym:crates/myrn-store/src/graph.rs#graph::Node::location@31f211a751fa3749
map_version: '2'
---
# Node::location

Formats the node's source position as 'path:start-end', 'path:start' or 'path'.

## Behaviour

- path 'x.rs', lines 3-7 -> 'x.rs:3-7'
- path 'x.rs', start 3 and end 3 -> 'x.rs:3'
- no path -> empty string; end without start -> just the path

## Terms

file position, line range, span, source location, file and line, path string

## Connections

- **Location:** `crates/myrn-store/src/graph.rs:62-69`
- **Signature:** `pub fn location(&self) -> String`
- **In:** [[file-crates-myrn-store-src-graph-rs]]
- **Called by:** [[engine-trace]], [[link-new]], [[symbolcard-render]], [[symbolcard-render-compact]], [[render-one-line]], [[engine-recall]], [[engine-review-items]], [[engine-map-body]], [[engine-changed]], [[ui-serve]]
