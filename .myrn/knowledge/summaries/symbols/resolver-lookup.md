---
id: resolver-lookup
type: summary
status: unverified
title: Resolver::lookup
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::lookup
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::lookup@925cb0ada4d548eb
---
# Resolver::lookup

Finds symbols by qualified name, excluding tests and other language families. With several candidates it prefers the `near` file, then the same Rust crate, and otherwise returns all of them.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:427-457`
- **Signature:** `fn lookup(&self, qname: &str, near: Option<&str>) -> Vec<usize>`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
- **Calls:** [[lang-family]], [[resolver-lookup-in-crate]]
- **Called by:** [[resolver-resolve-bare]], [[resolver-resolve-path]], [[resolver-follow-rust]], [[resolver-reexported]], [[resolver-exported-from]], [[resolver-resolve-method]]
