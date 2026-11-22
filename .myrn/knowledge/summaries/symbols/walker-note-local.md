---
id: walker-note-local
type: summary
status: unverified
title: Walker::note_local
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::note_local
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::note_local@bd4ed1e76c1c3f02
---
# Walker::note_local

Infers a local's type from an annotation or a constructor-like initializer (Type::new, Python Type(...), struct literal, `new Type`), unwrapping try/await first. Lowercase free functions in Rust are not treated as constructors.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:333-400`
- **Signature:** `fn note_local(&mut self, n: Node<'_>)`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Calls:** [[lang-family]], [[walker-text]], [[walker-bind-local]]
- **Called by:** [[walker-visit]]
