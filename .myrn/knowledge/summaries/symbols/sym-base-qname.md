---
id: sym-base-qname
type: summary
status: unverified
title: Sym::base_qname
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Sym::base_qname
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Sym::base_qname@f54a738cc1955dcc
---
# Sym::base_qname

Returns qname with any trailing `#<digits>` suffix removed; the Resolver indexes symbols by this value.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:34-36`
- **Signature:** `fn base_qname(&self) -> &str`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
- **Calls:** [[symbols-strip-suffix]]
- **Called by:** [[sym-parent]], [[resolver-new]], [[resolver-resolve-bare]]
