---
id: parse-import
type: summary
status: unverified
title: parse::Import
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Import
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Import@912c007ec87cb456
map_version: '2'
---
# parse::Import

One imported name: the module spec, the local name it binds, the line, and for Rust the module path of the use site.

## Behaviour

- Rust use a::{B, c as C} gives spec 'a::B' alias 'B' and spec 'a::c' alias 'C'
- Python 'from .store import lookup' gives spec '.store.lookup'; 'from . import x' gives '.x'
- JS bare import of './p' gives spec './p#'; 'import * as u' from './m' gives './m#*' alias 'u'

## Terms

import record, use statement, dependency, binding, alias, import spec, require

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:73-79`
- **Signature:** `pub struct Import`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
