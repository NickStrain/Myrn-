---
id: walker-js-import
type: summary
status: unverified
title: Walker::js_import
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::js_import
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::js_import@9f5c542b19e223eb
map_version: '2'
---
# Walker::js_import

Records one Import per default, namespace or named binding of an ES import statement, and a bare side-effect Import when there are none.

## Behaviour

- import { lookup as find } from './store' gives spec './store#lookup' alias 'find'
- import x, * as u from 'm' gives 'm#default' alias 'x' and 'm#*' alias 'u'
- import './polyfill' gives spec './polyfill#' with alias None

## Terms

import statement, ES import, default import, named import, namespace import, side effect import, bare import

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:1074-1144`
- **Signature:** `fn js_import(&mut self, n: Node<'_>)`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Calls:** [[walker-text]], [[walker-field-text]], [[walker-line]], [[walker-push-import]], [[parse-unquote]]
- **Called by:** [[walker-classify-js]]
