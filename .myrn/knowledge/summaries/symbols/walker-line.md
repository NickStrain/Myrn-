---
id: walker-line
type: summary
status: unverified
title: Walker::line
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::line
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::line@6fc6ed7b85c9cb60
---
# Walker::line

Returns the 1-based start line of a node.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:185-187`
- **Signature:** `fn line(n: Node<'_>) -> i64`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Called by:** [[walker-rust-macro-calls]], [[walker-visit]], [[walker-push-def]], [[walker-classify-rust]], [[walker-classify-python]], [[walker-classify-js]], [[walker-js-import]], [[walker-js-require]]
