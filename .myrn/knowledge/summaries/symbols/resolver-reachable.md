---
id: resolver-reachable
type: summary
status: unverified
title: Resolver::reachable
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::reachable
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::reachable@d12d4c37beaa9751
---
# Resolver::reachable

True when two files are the same, share a Rust crate, or are linked by one direct import or a second-hop import.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:828-844`
- **Signature:** `fn reachable(&self, from: &str, to: &str) -> bool`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
- **Calls:** [[lang-from-path]], [[lang-rust-module]]
- **Called by:** [[resolver-resolve-bare]], [[resolver-resolve-path]], [[resolver-resolve-method]]
