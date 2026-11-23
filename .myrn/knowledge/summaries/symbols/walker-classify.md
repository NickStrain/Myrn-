---
id: walker-classify
type: summary
status: unverified
title: Walker::classify
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::classify
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::classify@ad730fa427a4cf55
map_version: '2'
---
# Walker::classify

Routes a node to the classifier for the file's language family: Rust, Python, or JS/TS.

## Behaviour

- Rust files use classify_rust, Python files classify_python, and JavaScript and TypeScript files classify_js
- Adds no logic of its own

## Terms

dispatch, language router, node classifier, family

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:589-595`
- **Signature:** `fn classify<'t>(&mut self, n: Node<'t>) -> Action<'t>`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Calls:** [[lang-family]], [[walker-classify-rust]], [[walker-classify-python]], [[walker-classify-js]]
- **Called by:** [[walker-visit]]
