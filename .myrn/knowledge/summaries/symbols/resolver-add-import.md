---
id: resolver-add-import
type: summary
status: unverified
title: Resolver::add_import
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::add_import
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::add_import@055265b6023848e7
---
# Resolver::add_import

Records one import (spec, optional alias, scope) and returns the target file if it is in the repository. A named import that cannot be resolved gets an external binding so later calls through it are not guessed, and `*` aliases become whole-file namespace bindings.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:226-283`
- **Signature:** `pub fn add_import( &mut self, file: &str, spec: &str, alias: Option<&str>, scope: Option<&str>, ) -> Option<String>`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
- **Calls:** [[lang-from-path]], [[lang-family]], [[resolver-rust-import]], [[resolver-py-import]], [[resolver-js-import]]
- **Called by:** [[symbols-resolve-all]]
- **Tests:** `rust_paths_imports_and_self`, `external_imports_block_guesses`, `python_imports_and_methods`, `js_relative_imports_and_inference`
