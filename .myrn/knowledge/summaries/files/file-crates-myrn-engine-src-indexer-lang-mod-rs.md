---
id: file-crates-myrn-engine-src-indexer-lang-mod-rs
type: summary
status: unverified
title: crates/myrn-engine/src/indexer/lang/mod.rs
source: myrn summarize
about: file:crates/myrn-engine/src/indexer/lang/mod.rs
anchors:
- file:crates/myrn-engine/src/indexer/lang/mod.rs@c5f74b41422b7d76
---
# crates/myrn-engine/src/indexer/lang/mod.rs

Defines the five supported languages (Rust, Python, JavaScript, TypeScript, Tsx) with file-extension detection, tree-sitter grammar lookup, qualified-name separators, and per-family rules that turn a file path into module segments or a test-file flag. The rule helpers are rust_module, python_module, module_prefix and is_python_test_file.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/lang/mod.rs`
- **In:** [[dir-crates-myrn-engine-src-indexer-lang]]
- **Defines:** [[lang-lang]], [[lang-from-path]], [[lang-name]], [[lang-parse-name]], [[lang-grammar]], [[lang-family]], [[lang-sep]], [[lang-family-mod]], [[lang-rust-module]], [[lang-python-module]], [[lang-module-prefix]], [[lang-is-python-test-file]]
- **Imported by:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-engine-src-indexer-body-hash-rs]], [[file-crates-myrn-engine-src-indexer-git-rs]], [[file-crates-myrn-engine-src-indexer-mod-rs]], [[file-crates-myrn-engine-src-indexer-parse-rs]], [[file-crates-myrn-engine-src-indexer-symbols-rs]], [[file-crates-myrn-engine-src-indexer-walk-rs]]
