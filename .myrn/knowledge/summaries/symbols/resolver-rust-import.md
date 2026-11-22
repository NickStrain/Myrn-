---
id: resolver-rust-import
type: summary
status: unverified
title: Resolver::rust_import
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::rust_import
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::rust_import@0f06c1c87b8ebe5a
---
# Resolver::rust_import

Splits a Rust `use` spec, turning a trailing `::*` into a glob flag, and resolves the path through rust_path to (file, qname, glob).

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:340-353`
- **Signature:** `fn rust_import( &self, file: &str, spec: &str, scope: Option<&str>, ) -> Option<(String, Option<String>, bool)>`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
- **Calls:** [[resolver-rust-path]]
- **Called by:** [[resolver-add-import]]
