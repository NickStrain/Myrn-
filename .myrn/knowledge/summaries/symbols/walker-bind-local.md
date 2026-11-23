---
id: walker-bind-local
type: summary
status: unverified
title: Walker::bind_local
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::bind_local
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::bind_local@e0bea5f790b988da
map_version: '2'
---
# Walker::bind_local

Stores a variable name to type name mapping when the pattern is a plain identifier and the type passes type_name.

## Behaviour

- mut r binds r; tuple and destructuring patterns are ignored
- &mut Engine binds to Engine; Vec<Foo> binds nothing
- A later bind of the same name overwrites the earlier type

## Terms

bind variable, record local, local binding, variable type, name to type

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:402-416`
- **Signature:** `fn bind_local(&mut self, name: Node<'_>, ty: Node<'_>)`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Calls:** [[walker-text]], [[parse-type-name]]
- **Called by:** [[walker-note-params]], [[walker-note-local]]
