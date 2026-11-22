---
id: walker-classify-rust
type: summary
status: unverified
title: Walker::classify_rust
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::classify_rust
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::classify_rust@91233a1321949c3b
---
# Walker::classify_rust

Maps Rust nodes to defs (fn, struct, enum, trait, const, static, macro, and others), impl and inline-mod scopes, use/mod imports, and call-expression refs; function_item is a method inside an impl or trait, and macro invocations are handled by rust_macro_calls.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:599-714`
- **Signature:** `fn classify_rust<'t>(&mut self, n: Node<'t>) -> Action<'t>`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Calls:** [[walker-text]], [[walker-field-text]], [[walker-line]], [[walker-push-import]], [[walker-rust-macro-calls]], [[walker-in-type-scope]], [[walker-rust-type-name]], [[walker-rust-has-test-attr]], [[walker-rust-use-paths]], [[parse-truncate]], [[tests-def]]
- **Called by:** [[walker-classify]]
