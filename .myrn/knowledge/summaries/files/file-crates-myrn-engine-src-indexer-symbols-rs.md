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

Cross-file resolver that turns stored raw references and import rows into calls, imports and tests edges, marking each resolved (scope or import based) or inferred (unique name match) and dropping ambiguous ones. Its main parts are the Sym and RawRef inputs, the Resolver with per-language import handling (Rust, Python, JS) and lookup and resolve routines, the resolve_all entry point that rebuilds edges in the database, and unit tests.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs`
- **In:** [[dir-crates-myrn-engine-src-indexer]]
- **Defines:** [[symbols-sym]], [[sym-base-qname]], [[sym-parent]], [[sym-callable]], [[symbols-strip-suffix]], [[symbols-binding]], [[binding-external]], [[symbols-rawref]], [[symbols-resolver]], [[resolver-new]], [[resolver-sym]], [[resolver-add-import]], [[resolver-rust-path]], [[resolver-rust-import]], [[resolver-py-import]], [[resolver-js-import]], [[resolver-lookup]], [[resolver-lookup-in-crate]], [[resolver-resolve]], [[resolver-resolved]], [[resolver-binding]], [[resolver-resolve-bare]], [[resolver-resolve-path]], [[resolver-follow-rust]], [[resolver-reexported]]
- **Imports:** [[file-crates-myrn-engine-src-indexer-lang-mod-rs]], [[file-crates-myrn-engine-src-indexer-parse-rs]], [[file-crates-myrn-model-src-id-rs]], [[file-crates-myrn-model-src-lib-rs]]
- **Imported by:** [[file-crates-myrn-engine-src-indexer-mod-rs]]
