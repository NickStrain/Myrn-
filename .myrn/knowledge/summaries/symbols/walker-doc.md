---
id: walker-doc
type: summary
status: unverified
title: Walker::doc
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::doc
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::doc@9621059d3fc2d401
---
# Walker::doc

Extracts the doc comment: Python first-statement string, Rust contiguous /// or /** lines above the item (skipping attributes), JS /** above the item or its export wrapper; keeps the first paragraph, collapsed to one line, max 300 chars.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:529-587`
- **Signature:** `fn doc(&self, node: Node<'_>, anchor: Option<Node<'_>>) -> Option<String>`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Calls:** [[lang-family]], [[walker-text]], [[parse-truncate]], [[parse-unquote]], [[parse-clean-block-comment]], [[sym-parent]]
- **Called by:** [[walker-push-def]]
