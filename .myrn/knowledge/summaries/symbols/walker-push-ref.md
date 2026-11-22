---
id: walker-push-ref
type: summary
status: unverified
title: Walker::push_ref
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::push_ref
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::push_ref@ee66a05013457d21
---
# Walker::push_ref

Records a Ref against the enclosing def. A Method call whose receiver is a known local with a type is rewritten to a Path call on that type; refs with no enclosing def or an empty name are dropped.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:209-234`
- **Signature:** `fn push_ref( &mut self, mut kind: RefKind, name: String, mut qualifier: Option<String>, line: i64, )`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Called by:** [[walker-rust-macro-calls]], [[walker-visit]]
