---
id: walker-visit
type: summary
status: unverified
title: Walker::visit
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::visit
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::visit@27476234a0414896
map_version: '2'
---
# Walker::visit

Visits one node depth-first: notes locals, classifies the node, and acts on the result by recording a def, scope or reference, skipping, or descending.

## Behaviour

- Depth above 512 returns immediately, silently dropping that subtree
- Def: pushes its name as a scope, sets itself as enclosing, visits children, then restores locals, enclosing and scope
- Ref: records the reference and still descends into the children

## Terms

traversal, recursive walk, node dispatch, tree visit, depth limit, descend

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:422-460`
- **Signature:** `fn visit(&mut self, node: Node<'_>, depth: usize)`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Calls:** [[walker-line]], [[walker-push-ref]], [[walker-note-params]], [[walker-note-local]], [[walker-visit-children]], [[walker-push-def]], [[walker-classify]]
- **Called by:** [[fileparser-parse]], [[walker-visit-children]]
