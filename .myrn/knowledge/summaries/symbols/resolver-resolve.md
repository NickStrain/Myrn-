---
id: resolver-resolve
type: summary
status: unverified
title: Resolver::resolve
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::resolve
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::resolve@355bc3a023517900
---
# Resolver::resolve

Entry point for one raw reference: dispatches on its kind (unknown kinds count as call) to resolve_bare, resolve_path or resolve_method, then removes duplicate targets.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:468-485`
- **Signature:** `pub fn resolve(&self, src: usize, r: &RawRef) -> Vec<(usize, EdgeStatus)>`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
- **Calls:** [[refkind-parse]], [[resolver-resolve-bare]], [[resolver-resolve-path]], [[resolver-resolve-method]]
- **Called by:** [[symbols-resolve-all]], [[tests-targets]]
