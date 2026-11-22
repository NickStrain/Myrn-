---
id: walker-visit
type: summary
status: unverified
title: Walker::visit
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::visit
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::visit@27476234a0414896
---
# Walker::visit

Core recursive dispatch: records locals inside a def, classifies the node, and for defs saves and restores the locals map and enclosing index around the children; recursion stops past depth 512.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:422-460`
- **Signature:** `fn visit(&mut self, node: Node<'_>, depth: usize)`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Calls:** [[walker-line]], [[walker-push-ref]], [[walker-note-params]], [[walker-note-local]], [[walker-visit-children]], [[walker-push-def]], [[walker-classify]]
- **Called by:** [[fileparser-parse]], [[walker-visit-children]]
