---
id: resolver-py-import
type: summary
status: unverified
title: Resolver::py_import
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::py_import
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::py_import@7a12987a558fa716
---
# Resolver::py_import

Resolves a Python import including leading-dot relative forms and `*` globs to (file, qname, glob). For absolute names with no direct module match, it falls back to a single module whose dotted path ends with the name, which covers source roots below the repository root.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:355-396`
- **Signature:** `fn py_import(&self, file: &str, spec: &str) -> Option<(String, Option<String>, bool)>`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
- **Calls:** [[lang-python-module]]
- **Called by:** [[resolver-add-import]]
