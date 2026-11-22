---
id: lang-python-module
type: summary
status: unverified
title: lang::python_module
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/lang/mod.rs#indexer::lang::python_module
anchors:
- sym:crates/myrn-engine/src/indexer/lang/mod.rs#indexer::lang::python_module@f07c8fb18e2984db
---
# lang::python_module

Converts a Python file path to its dotted module segments by stripping the .py or .pyi suffix and splitting on slashes. It removes a leading src segment when more segments follow and a trailing __init__, so src/pkg/__init__.py becomes ["pkg"].

## Connections

- **Location:** `crates/myrn-engine/src/indexer/lang/mod.rs:120-130`
- **Signature:** `pub fn python_module(path: &str) -> Vec<String>`
- **In:** [[file-crates-myrn-engine-src-indexer-lang-mod-rs]]
- **Called by:** [[lang-module-prefix]], [[resolver-new]], [[resolver-py-import]], [[resolver-is-top-level]]
- **Tests:** `python_modules`
