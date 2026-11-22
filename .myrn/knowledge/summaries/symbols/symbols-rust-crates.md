---
id: symbols-rust-crates
type: summary
status: unverified
title: symbols::rust_crates
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::rust_crates
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::rust_crates@f48189a1575e5ae3
---
# symbols::rust_crates

Reads each Rust crate directory's Cargo.toml for its package name, with hyphens turned into underscores, and falls back to the last directory segment. Returns a map from crate name to directory.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:906-931`
- **Signature:** `fn rust_crates(root: &Path, files: &[(String, Lang)]) -> HashMap<String, String>`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
- **Calls:** [[lang-rust-module]]
- **Called by:** [[symbols-resolve-all]]
