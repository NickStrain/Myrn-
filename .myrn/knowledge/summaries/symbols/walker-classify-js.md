---
id: walker-classify-js
type: summary
status: unverified
title: Walker::classify_js
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::classify_js
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::classify_js@7ad8f5ea1786a3b8
map_version: '2'
---
# Walker::classify_js

Maps JS and TypeScript declarations, arrow functions bound to const, describe/it/test blocks, calls, new expressions and import/export statements to actions.

## Behaviour

- describe('refresh', ...) opens a Module scope; it('returns', ...) makes a test def named 'returns' nested under 'refresh'
- const f = () => ... becomes a function def; const x = require('m') is handed to js_require
- Any export with a source, including named re-exports, records spec 'm#*'

## Terms

TypeScript classification, describe block, test title, arrow function, re-export, new expression, class declaration

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:919-1072`
- **Signature:** `fn classify_js<'t>(&mut self, n: Node<'t>) -> Action<'t>`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Calls:** [[walker-text]], [[walker-field-text]], [[walker-line]], [[walker-push-import]], [[walker-js-import]], [[walker-js-require]], [[parse-truncate]], [[parse-unquote]], [[tests-def]]
- **Called by:** [[walker-classify]]
