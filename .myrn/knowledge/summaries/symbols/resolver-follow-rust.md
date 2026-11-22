---
id: resolver-follow-rust
type: summary
status: unverified
title: Resolver::follow_rust
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::follow_rust
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::follow_rust@d6f0fb0665d4f20e
---
# Resolver::follow_rust

Looks up a Rust qualified name inside a module file and follows `pub use` bindings there, up to the given depth.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:643-671`
- **Signature:** `fn follow_rust(&self, file: &str, qname: &str, depth: usize) -> Vec<usize>`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
- **Calls:** [[lang-rust-module]], [[resolver-lookup]], [[resolver-binding]], [[resolver-follow-rust]]
- **Called by:** [[resolver-resolve-path]], [[resolver-follow-rust]]
