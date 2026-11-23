---
id: lang-python-module
type: summary
status: unverified
title: lang::python_module
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/lang/mod.rs#indexer::lang::python_module
anchors:
- sym:crates/myrn-engine/src/indexer/lang/mod.rs#indexer::lang::python_module@f07c8fb18e2984db
map_version: '2'
---
# lang::python_module

Turns a Python file path into dotted module segments. It drops a leading src directory and a trailing __init__, and treats .pyi the same as .py.

## Behaviour

- pkg/a.py -> [pkg, a]
- src/pkg/__init__.py -> [pkg]
- src/__init__.py -> [], because src is dropped only when more than one segment remains

## Terms

dotted module path, module name, package path, __init__, python module

## Connections

- **Location:** `crates/myrn-engine/src/indexer/lang/mod.rs:120-130`
- **Signature:** `pub fn python_module(path: &str) -> Vec<String>`
- **In:** [[file-crates-myrn-engine-src-indexer-lang-mod-rs]]
- **Called by:** [[lang-module-prefix]], [[resolver-new]], [[resolver-py-import]], [[resolver-is-top-level]]
- **Tests:** `python_modules`
