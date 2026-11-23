---
id: walker-rust-type-name
type: summary
status: unverified
title: Walker::rust_type_name
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::rust_type_name
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::rust_type_name@f9aafdb5583d4fec
map_version: '2'
---
# Walker::rust_type_name

Reduces a Rust type node to its bare type name for use as an impl target or local type.

## Behaviour

- Wrapper<T> gives 'Wrapper'; &mut crate::Engine gives 'Engine'
- Scoped paths keep only the last segment
- Other node kinds, such as [u8], fall back to their source text with generics removed

## Terms

type name, base type, impl target, normalized type, bare name

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:716-729`
- **Signature:** `fn rust_type_name(&self, t: Node<'_>) -> String`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Calls:** [[walker-text]], [[walker-field-text]], [[walker-rust-type-name]], [[parse-strip-generics]]
- **Called by:** [[walker-classify-rust]], [[walker-rust-type-name]]
