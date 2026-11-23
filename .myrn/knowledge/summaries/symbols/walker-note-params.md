---
id: walker-note-params
type: summary
status: unverified
title: Walker::note_params
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::note_params
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::note_params@5471687166215b3d
map_version: '2'
---
# Walker::note_params

Records the declared type of each typed parameter so later calls on that name can be qualified by type.

## Behaviour

- Rust fn f(engine: &mut Engine) binds engine to Engine; Python def f(db: Db) binds db to Db
- Untyped parameters are ignored, and so are types that type_name rejects such as Vec<T>
- For JS arrows it reads parameters from the declarator's value, as in const f = (x: T) => ...

## Terms

parameter types, typed params, argument types, local bindings, signature types

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:299-330`
- **Signature:** `fn note_params(&mut self, def: Node<'_>)`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Calls:** [[walker-bind-local]]
- **Called by:** [[walker-visit]]
