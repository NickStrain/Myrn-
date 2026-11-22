---
id: sym-callable
type: summary
status: unverified
title: Sym::callable
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Sym::callable
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Sym::callable@678d80abae931bb3
---
# Sym::callable

True for non-test function, method, class or struct symbols, the only kinds that can be call targets.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:47-53`
- **Signature:** `fn callable(&self) -> bool`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
- **Calls:** [[kind-as-str]]
- **Called by:** [[resolver-resolve-bare]], [[resolver-resolve-path]], [[resolver-resolve-method]]
