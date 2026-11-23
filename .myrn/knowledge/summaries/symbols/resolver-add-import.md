---
id: resolver-add-import
type: summary
status: unverified
title: Resolver::add_import
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::add_import
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::add_import@055265b6023848e7
map_version: '2'
---
# Resolver::add_import

Records one import in the import graph and bindings, and returns the imported repository file if one was found.

## Behaviour

- file extension not supported -> None, nothing recorded
- unresolved spec with an alias not '*' -> records an external binding and returns None
- 'import * as R' (glob with alias) -> binds R to the whole target file

## Terms

import recording, alias, wildcard import, namespace import, external import, import graph

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:226-283`
- **Signature:** `pub fn add_import( &mut self, file: &str, spec: &str, alias: Option<&str>, scope: Option<&str>, ) -> Option<String>`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
- **Calls:** [[lang-from-path]], [[lang-family]], [[resolver-rust-import]], [[resolver-py-import]], [[resolver-js-import]]
- **Called by:** [[symbols-resolve-all]]
- **Tests:** `rust_paths_imports_and_self`, `external_imports_block_guesses`, `python_imports_and_methods`, `js_relative_imports_and_inference`
