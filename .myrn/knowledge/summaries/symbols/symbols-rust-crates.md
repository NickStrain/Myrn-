---
id: symbols-rust-crates
type: summary
status: unverified
title: symbols::rust_crates
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::rust_crates
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::rust_crates@f48189a1575e5ae3
map_version: '2'
---
# symbols::rust_crates

Reads each Rust crate's Cargo.toml under the repository root and maps its underscored package name to its directory, falling back to the last directory segment.

## Behaviour

- package name 'my-lib' in crates/my-lib -> key 'my_lib' mapped to 'crates/my-lib'
- missing or unreadable Cargo.toml -> directory's last segment is used
- reads files from disk

## Terms

crate name, Cargo.toml, package name, crate map, underscore name, manifest

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:906-931`
- **Signature:** `fn rust_crates(root: &Path, files: &[(String, Lang)]) -> HashMap<String, String>`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
- **Calls:** [[lang-rust-module]]
- **Called by:** [[symbols-resolve-all]]
