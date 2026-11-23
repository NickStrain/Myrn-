---
id: walker-line
type: summary
status: unverified
title: Walker::line
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::line
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::line@6fc6ed7b85c9cb60
map_version: '2'
---
# Walker::line

Gives the 1-based line number where a node starts.

## Behaviour

- Node on the first row of the file -> 1
- Node starting on 0-based row 9 -> 10
- Used for start_line on defs and for the line stored on refs and imports

## Terms

line number, start line, row, position, source line

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:185-187`
- **Signature:** `fn line(n: Node<'_>) -> i64`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Called by:** [[walker-rust-macro-calls]], [[walker-visit]], [[walker-push-def]], [[walker-classify-rust]], [[walker-classify-python]], [[walker-classify-js]], [[walker-js-import]], [[walker-js-require]]
