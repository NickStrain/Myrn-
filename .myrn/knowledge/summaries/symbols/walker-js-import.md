---
id: walker-js-import
type: summary
status: unverified
title: Walker::js_import
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::js_import
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::js_import@9f5c542b19e223eb
---
# Walker::js_import

Creates one import per binding in an import statement (default '#default', namespace '#*', named '#member'), or a bare '#' import when none are bound.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:1074-1144`
- **Signature:** `fn js_import(&mut self, n: Node<'_>)`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Calls:** [[walker-text]], [[walker-field-text]], [[walker-line]], [[walker-push-import]], [[parse-unquote]]
- **Called by:** [[walker-classify-js]]
