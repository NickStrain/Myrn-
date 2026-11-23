---
id: walker-visit-children
type: summary
status: unverified
title: Walker::visit_children
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::visit_children
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::visit_children@30d78d6d7cddb58c
map_version: '2'
---
# Walker::visit_children

Visits every named child of a node one level deeper.

## Behaviour

- Anonymous tokens such as punctuation and keywords are never visited
- Children are collected into a Vec first so the tree cursor is not held across recursion
- depth is incremented by one per call

## Terms

named children, recurse, child nodes, descend, nested nodes

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:462-468`
- **Signature:** `fn visit_children(&mut self, node: Node<'_>, depth: usize)`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Calls:** [[walker-visit]]
- **Called by:** [[walker-visit]]
