---
id: lang-module-prefix
type: summary
status: unverified
title: lang::module_prefix
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/lang/mod.rs#indexer::lang::module_prefix
anchors:
- sym:crates/myrn-engine/src/indexer/lang/mod.rs#indexer::lang::module_prefix@defe441410c574b0
---
# lang::module_prefix

Returns the module-path segments that prefix qualified names in a file, dispatching on the language family. Rust uses rust_module (crate directory discarded), Python uses python_module, and Js always returns an empty vector.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/lang/mod.rs:133-139`
- **Signature:** `pub fn module_prefix(lang: Lang, path: &str) -> Vec<String>`
- **In:** [[file-crates-myrn-engine-src-indexer-lang-mod-rs]]
- **Calls:** [[lang-family]], [[lang-rust-module]], [[lang-python-module]]
- **Called by:** [[fileparser-parse]]
