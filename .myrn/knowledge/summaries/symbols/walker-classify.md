---
id: walker-classify
type: summary
status: unverified
title: Walker::classify
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::classify
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::classify@ad730fa427a4cf55
---
# Walker::classify

Dispatches node classification to the Rust, Python, or JS/TS classifier based on the language family.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:589-595`
- **Signature:** `fn classify<'t>(&mut self, n: Node<'t>) -> Action<'t>`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Calls:** [[lang-family]], [[walker-classify-rust]], [[walker-classify-python]], [[walker-classify-js]]
- **Called by:** [[walker-visit]]
