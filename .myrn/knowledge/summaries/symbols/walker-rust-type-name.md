---
id: walker-rust-type-name
type: summary
status: unverified
title: Walker::rust_type_name
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::rust_type_name
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::rust_type_name@f9aafdb5583d4fec
---
# Walker::rust_type_name

Reduces a Rust type node to a bare name: strips generics, references, and pointers, and takes the last segment of scoped types.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:716-729`
- **Signature:** `fn rust_type_name(&self, t: Node<'_>) -> String`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Calls:** [[walker-text]], [[walker-field-text]], [[walker-rust-type-name]], [[parse-strip-generics]]
- **Called by:** [[walker-classify-rust]], [[walker-rust-type-name]]
