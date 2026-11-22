---
id: binding-external
type: summary
status: unverified
title: Binding::external
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Binding::external
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Binding::external@f2128a5a9d6ffda8
---
# Binding::external

True when both file and qname are None, meaning the import points outside the repository.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:73-75`
- **Signature:** `fn external(&self) -> bool`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
- **Called by:** [[resolver-resolve-bare]], [[resolver-resolve-path]]
