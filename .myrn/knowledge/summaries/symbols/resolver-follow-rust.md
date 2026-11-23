---
id: resolver-follow-rust
type: summary
status: unverified
title: Resolver::follow_rust
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::follow_rust
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::follow_rust@d6f0fb0665d4f20e
map_version: '2'
---
# Resolver::follow_rust

Looks up a Rust qualified name in a file and, if missing, follows a use binding for its first segment, up to depth hops, to reach re-exported items.

## Behaviour

- depth 0 -> only the direct lookup
- item re-exported by pub use in lib.rs -> followed to the defining module
- no matching binding -> empty

## Terms

re-export, pub use, use chain, re-exported item, hop limit

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:643-671`
- **Signature:** `fn follow_rust(&self, file: &str, qname: &str, depth: usize) -> Vec<usize>`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
- **Calls:** [[lang-rust-module]], [[resolver-lookup]], [[resolver-binding]], [[resolver-follow-rust]]
- **Called by:** [[resolver-resolve-path]], [[resolver-follow-rust]]
