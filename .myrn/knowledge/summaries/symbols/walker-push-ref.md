---
id: walker-push-ref
type: summary
status: unverified
title: Walker::push_ref
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::push_ref
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::push_ref@ee66a05013457d21
map_version: '2'
---
# Walker::push_ref

Records a Ref from the current enclosing definition, turning a Method call into a Path when the receiver is a known local type.

## Behaviour

- s.get() where s was bound to Store -> Path with qualifier 'Store'
- A receiver with no known type stays Method, with its receiver text as qualifier
- No enclosing definition, or an empty name -> nothing recorded

## Terms

record call, add reference, local type rewrite, call site, receiver resolution

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:209-234`
- **Signature:** `fn push_ref( &mut self, mut kind: RefKind, name: String, mut qualifier: Option<String>, line: i64, )`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Called by:** [[walker-rust-macro-calls]], [[walker-visit]]
