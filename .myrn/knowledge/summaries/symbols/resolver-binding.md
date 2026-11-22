---
id: resolver-binding
type: summary
status: unverified
title: Resolver::binding
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::binding
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::binding@dc8851a1d68ce84c
---
# Resolver::binding

Returns the import binding for a local name in a file, if one exists.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:491-493`
- **Signature:** `fn binding(&self, file: &str, name: &str) -> Option<&Binding>`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
- **Called by:** [[resolver-resolve-bare]], [[resolver-resolve-path]], [[resolver-follow-rust]], [[resolver-reexported]], [[resolver-exported-from]], [[resolver-resolve-method]]
