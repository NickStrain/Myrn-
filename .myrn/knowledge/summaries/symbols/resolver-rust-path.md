---
id: resolver-rust-path
type: summary
status: unverified
title: Resolver::rust_path
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::rust_path
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::rust_path@d550d2b9a3b43452
---
# Resolver::rust_path

Resolves a Rust segment list anchored at crate, self, super, a known crate name, or the current module scope to (module file, full qname). Unanchored paths try the scope and then the crate root, and a match is accepted only if the symbol exists in that crate.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:286-338`
- **Signature:** `fn rust_path( &self, file: &str, scope: Option<&str>, segs: &[&str], ) -> Option<(String, String)>`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
- **Calls:** [[lang-rust-module]], [[resolver-lookup-in-crate]]
- **Called by:** [[resolver-rust-import]], [[resolver-resolve-path]]
