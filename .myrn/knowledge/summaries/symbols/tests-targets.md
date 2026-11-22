---
id: tests-targets
type: summary
status: unverified
title: tests::targets
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::tests::targets
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::tests::targets@2be50c1e0a3a5f70
---
# tests::targets

Test helper that resolves one RawRef from a source Sym and returns (qname, status) pairs.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:1080-1086`
- **Signature:** `fn targets(r: &Resolver, src: &Sym, raw: RawRef) -> Vec<(String, EdgeStatus)>`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
- **Calls:** [[resolver-sym]], [[resolver-resolve]], [[resolver-index-of]]
- **Tests:** `rust_paths_imports_and_self`, `external_imports_block_guesses`, `python_imports_and_methods`, `js_relative_imports_and_inference`
