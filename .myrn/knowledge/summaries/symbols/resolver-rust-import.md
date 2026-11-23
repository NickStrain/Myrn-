---
id: resolver-rust-import
type: summary
status: unverified
title: Resolver::rust_import
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::rust_import
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::rust_import@0f06c1c87b8ebe5a
map_version: '2'
---
# Resolver::rust_import

Parses a Rust use spec, treats a trailing '::*' as a glob, and resolves the path with rust_path.

## Behaviour

- 'crate::a::B' -> (file of module a, Some('a::B'), false)
- 'crate::a::*' -> glob flag true, path 'crate::a'
- unresolvable path -> None

## Terms

use declaration, use spec, glob import, Rust import, use statement

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:340-353`
- **Signature:** `fn rust_import( &self, file: &str, spec: &str, scope: Option<&str>, ) -> Option<(String, Option<String>, bool)>`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
- **Calls:** [[resolver-rust-path]]
- **Called by:** [[resolver-add-import]]
