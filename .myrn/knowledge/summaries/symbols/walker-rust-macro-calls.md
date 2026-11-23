---
id: walker-rust-macro-calls
type: summary
status: unverified
title: Walker::rust_macro_calls
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::rust_macro_calls
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::rust_macro_calls@cf78a0401e2a0d87
map_version: '2'
---
# Walker::rust_macro_calls

Scans Rust macro token trees for call-like tokens, since tree-sitter leaves macro arguments unparsed, and records each as a reference.

## Behaviour

- assert!(refresh(&s)) -> Call 'refresh'
- assert_eq!(crate::a::f(1)) -> Path 'f' with qualifier 'crate::a'
- Identifiers if, while, match, return, in, Some, Ok and Err are skipped; nested token trees are scanned recursively

## Terms

macro arguments, token tree, macro calls, println calls, macro body, raw tokens

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:238-296`
- **Signature:** `fn rust_macro_calls(&mut self, tree: Node<'_>)`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Calls:** [[walker-text]], [[walker-line]], [[walker-push-ref]], [[walker-rust-macro-calls]]
- **Called by:** [[walker-rust-macro-calls]], [[walker-classify-rust]]
