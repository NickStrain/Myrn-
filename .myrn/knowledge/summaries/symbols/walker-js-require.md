---
id: walker-js-require
type: summary
status: unverified
title: Walker::js_require
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::js_require
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::js_require@b14752bd514f2bb7
---
# Walker::js_require

Handles `const x = require(...)` (as '#*') and `const { a, b } = require(...)` (one import per member) when the argument is a string literal.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:1147-1192`
- **Signature:** `fn js_require(&mut self, name: Node<'_>, call: Node<'_>)`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Calls:** [[walker-text]], [[walker-field-text]], [[walker-line]], [[walker-push-import]], [[parse-unquote]]
- **Called by:** [[walker-classify-js]]
