---
id: dir-crates-myrn-engine-src-indexer-lang
type: summary
status: unverified
title: crates/myrn-engine/src/indexer/lang/
source: myrn summarize
about: module:crates/myrn-engine/src/indexer/lang
anchors:
- module:crates/myrn-engine/src/indexer/lang@f730b6531ab78628
---
# crates/myrn-engine/src/indexer/lang/

Language-registry and path-mapping layer for the indexer: `mod.rs` defines the supported languages (Rust, Python, JavaScript, TypeScript, TSX) and maps file paths to each language's tree-sitter grammar, resolution family, qualified-name separator, and module prefix. It also provides pure-string Rust and Python module-path helpers and a Python test-file detector, all without filesystem access.

## Connections

- **In:** [[dir-crates-myrn-engine-src-indexer]]
- **Files:** [[file-crates-myrn-engine-src-indexer-lang-mod-rs]]
