---
id: walker-classify-js
type: summary
status: unverified
title: Walker::classify_js
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::classify_js
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::classify_js@7ad8f5ea1786a3b8
---
# Walker::classify_js

Maps JS/TS functions, classes, methods, interfaces, types, enums, namespaces, and arrow or function-valued variables to defs; `test`/`it`/`describe`/`suite` string-titled calls (with only/skip/concurrent/todo modifiers) become test defs or scopes; re-exports become '#*' imports.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:919-1072`
- **Signature:** `fn classify_js<'t>(&mut self, n: Node<'t>) -> Action<'t>`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Calls:** [[walker-text]], [[walker-field-text]], [[walker-line]], [[walker-push-import]], [[walker-js-import]], [[walker-js-require]], [[parse-truncate]], [[parse-unquote]], [[tests-def]]
- **Called by:** [[walker-classify]]
