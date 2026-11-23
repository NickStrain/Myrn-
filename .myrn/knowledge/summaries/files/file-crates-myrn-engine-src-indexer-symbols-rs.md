---
id: file-crates-myrn-engine-src-indexer-symbols-rs
type: summary
status: unverified
title: crates/myrn-engine/src/indexer/symbols.rs
source: myrn summarize
about: file:crates/myrn-engine/src/indexer/symbols.rs
anchors:
- file:crates/myrn-engine/src/indexer/symbols.rs@73f6b69c95cad09f
---
# crates/myrn-engine/src/indexer/symbols.rs

Cross-file resolver that turns raw references and imports stored in SQLite into calls, imports and tests edges, with Rust, Python and JavaScript name rules. Resolver holds the lookup tables and per-file bindings, and resolve_all rebuilds those edges after each refresh.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs`
- **In:** [[dir-crates-myrn-engine-src-indexer]]
- **Defines:** [[symbols-sym]], [[sym-base-qname]], [[sym-parent]], [[sym-callable]], [[symbols-strip-suffix]], [[symbols-binding]], [[binding-external]], [[symbols-rawref]], [[symbols-resolver]], [[resolver-new]], [[resolver-sym]], [[resolver-add-import]], [[resolver-rust-path]], [[resolver-rust-import]], [[resolver-py-import]], [[resolver-js-import]], [[resolver-lookup]], [[resolver-lookup-in-crate]], [[resolver-resolve]], [[resolver-resolved]], [[resolver-binding]], [[resolver-resolve-bare]], [[resolver-resolve-path]], [[resolver-follow-rust]], [[resolver-reexported]]
- **Imports:** [[file-crates-myrn-engine-src-indexer-lang-mod-rs]], [[file-crates-myrn-engine-src-indexer-parse-rs]], [[file-crates-myrn-model-src-id-rs]], [[file-crates-myrn-model-src-lib-rs]]
- **Imported by:** [[file-crates-myrn-engine-src-indexer-mod-rs]]
