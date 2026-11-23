---
id: walker-js-require
type: summary
status: unverified
title: Walker::js_require
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::js_require
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::js_require@b14752bd514f2bb7
map_version: '2'
---
# Walker::js_require

Records CommonJS require bindings from a variable declarator: a plain name binds the whole module, and an object pattern binds each member.

## Behaviour

- const legacy = require('./legacy') gives spec './legacy#*' alias 'legacy'
- const { a, b: c } = require('./m') gives './m#a' alias 'a' and './m#b' alias 'c'
- const x = fetch('u') records nothing because the callee is not require

## Terms

commonjs, require call, destructured require, cjs import, require binding

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:1147-1192`
- **Signature:** `fn js_require(&mut self, name: Node<'_>, call: Node<'_>)`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Calls:** [[walker-text]], [[walker-field-text]], [[walker-line]], [[walker-push-import]], [[parse-unquote]]
- **Called by:** [[walker-classify-js]]
