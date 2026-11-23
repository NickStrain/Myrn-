---
id: resolver-py-import
type: summary
status: unverified
title: Resolver::py_import
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::py_import
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::py_import@7a12987a558fa716
map_version: '2'
---
# Resolver::py_import

Resolves a Python import spec, including relative dots and a trailing star, to a module file and dotted qualified name.

## Behaviour

- '.store.TokenStore' from pkg/auth/session.py -> pkg/auth/store.py with 'pkg.auth.store.TokenStore'
- 'app.x' with no exact module but one module ending in '.app.x' -> that file, with its source-root prefix
- '.*' or '*' suffix -> glob true

## Terms

Python import, relative import, dotted module, source root, star import, package depth

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:355-396`
- **Signature:** `fn py_import(&self, file: &str, spec: &str) -> Option<(String, Option<String>, bool)>`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
- **Calls:** [[lang-python-module]]
- **Called by:** [[resolver-add-import]]
