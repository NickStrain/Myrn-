---
id: walker-note-local
type: summary
status: unverified
title: Walker::note_local
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::note_local
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::note_local@bd4ed1e76c1c3f02
map_version: '2'
---
# Walker::note_local

Records a local variable's type from an explicit annotation or a constructor-like initializer, unwrapping try and await first.

## Behaviour

- let r = Resolver::new() binds r to Resolver; let x = make() binds nothing because make is not a constructor name
- let d: Db = open() binds d to Db; the annotation wins over the initializer
- Python x = Session() binds x to Session; a lowercase call such as x = load_data() binds nothing because type_name needs a capital letter

## Terms

local type, variable type, type inference, constructor call, let binding, struct literal, type hint

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:333-400`
- **Signature:** `fn note_local(&mut self, n: Node<'_>)`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Calls:** [[lang-family]], [[walker-text]], [[walker-bind-local]]
- **Called by:** [[walker-visit]]
