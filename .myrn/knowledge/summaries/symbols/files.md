---
id: files
type: summary
status: unverified
title: files
source: myrn summarize
about: sym:crates/myrn-engine/tests/sharing.rs#files
anchors:
- sym:crates/myrn-engine/tests/sharing.rs#files@4e44b6de1adf7138
map_version: '2'
---
# files

Returns the three fixture files for a sample Python package: an __init__.py that re-exports two modules, recipes.py holding the RECIPES text, and more.py with a chunked stub.

## Behaviour

- no input -> Vec of 3 (relative path, file content) pairs
- chunked in more.py returns [it] (a one-element list), not chunks
- RECIPES constant is the source of unique_everseen in pkg/recipes.py

## Terms

fixture, sample package, test files, pkg layout, re-export, starter files

## Connections

- **Location:** `crates/myrn-engine/tests/sharing.rs:11-20`
- **Signature:** `fn files() -> Vec<(&'static str, &'static str)>`
- **In:** [[file-crates-myrn-engine-tests-sharing-rs]]
- **Called by:** [[two-clones]]
