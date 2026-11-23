---
id: resolver-rust-path
type: summary
status: unverified
title: Resolver::rust_path
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::rust_path
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::rust_path@d550d2b9a3b43452
map_version: '2'
---
# Resolver::rust_path

Resolves a Rust path's segments against the current file and scope to the module file that defines it and the full qualified name.

## Behaviour

- 'crate::store::X' -> the store module file and 'store::X' in the same crate
- 'super::f' with scope 'a::b' -> 'a::f'
- unknown first segment -> tries the current module, then the crate root, requiring a lookup hit in the crate

## Terms

module path, use path, scope resolution, crate root, super and self, path segments

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:286-338`
- **Signature:** `fn rust_path( &self, file: &str, scope: Option<&str>, segs: &[&str], ) -> Option<(String, String)>`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
- **Calls:** [[lang-rust-module]], [[resolver-lookup-in-crate]]
- **Called by:** [[resolver-rust-import]], [[resolver-resolve-path]]
