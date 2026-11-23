---
id: walker-classify-rust
type: summary
status: unverified
title: Walker::classify_rust
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::classify_rust
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::classify_rust@91233a1321949c3b
map_version: '2'
---
# Walker::classify_rust

Maps Rust AST nodes to actions: items become defs, impl and inline mod become scopes, use and mod x; become imports, and calls and macro invocations become references. Comments and string literals are skipped.

## Behaviour

- impl<T> Display for Wrapper<T> opens a Type scope named 'Wrapper'
- Path::f() gives a Path reference with qualifier 'Path' (generics stripped); recv.f() gives a Method with receiver text cut to 80 characters
- A fn with #[test], #[tokio::test] or #[rstest] is marked is_test; a bodiless fn signature still becomes a def

## Terms

Rust classification, item kinds, struct enum trait, use declaration, impl block, call expression, macro invocation

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:599-714`
- **Signature:** `fn classify_rust<'t>(&mut self, n: Node<'t>) -> Action<'t>`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Calls:** [[walker-text]], [[walker-field-text]], [[walker-line]], [[walker-push-import]], [[walker-rust-macro-calls]], [[walker-in-type-scope]], [[walker-rust-type-name]], [[walker-rust-has-test-attr]], [[walker-rust-use-paths]], [[parse-truncate]], [[tests-def]]
- **Called by:** [[walker-classify]]
