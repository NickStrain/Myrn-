---
id: sym-parent
type: summary
status: unverified
title: Sym::parent
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Sym::parent
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Sym::parent@d1f879dcac4c98a5
---
# Sym::parent

Returns the enclosing scope's qualified name by cutting base_qname at the last language separator, or an empty string for top-level items.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:39-45`
- **Signature:** `fn parent(&self) -> &str`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
- **Calls:** [[lang-sep]], [[sym-base-qname]]
- **Called by:** [[engine-open-located]], [[git-index-commit]], [[walker-doc]], [[resolver-resolve-bare]], [[resolver-resolve-path]], [[resolver-resolve-method]], [[resolver-parent-is-type]], [[resolver-is-top-level]], [[write]], [[write-packing]], [[write-records]], [[write-summarize]]
