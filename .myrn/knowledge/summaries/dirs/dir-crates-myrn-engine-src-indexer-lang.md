---
id: dir-crates-myrn-engine-src-indexer-lang
type: summary
status: unverified
title: crates/myrn-engine/src/indexer/lang/
source: myrn summarize
about: module:crates/myrn-engine/src/indexer/lang
anchors:
- module:crates/myrn-engine/src/indexer/lang@dfb1cf9297285285
---
# crates/myrn-engine/src/indexer/lang/

Language registry for the indexer: the five supported languages (Rust, Python, JavaScript, TypeScript, Tsx) and the per-language rules the indexer uses to recognise files, pick a tree-sitter grammar, choose the qualified-name separator, and map a file path to module segments or a test-file flag. The rule helpers rust_module, python_module, module_prefix and is_python_test_file in mod.rs are the entry points other indexer code calls.

## Connections

- **In:** [[dir-crates-myrn-engine-src-indexer]]
- **Files:** [[file-crates-myrn-engine-src-indexer-lang-mod-rs]]
