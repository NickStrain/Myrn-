---
id: lang-module-prefix
type: summary
status: unverified
title: lang::module_prefix
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/lang/mod.rs#indexer::lang::module_prefix
anchors:
- sym:crates/myrn-engine/src/indexer/lang/mod.rs#indexer::lang::module_prefix@defe441410c574b0
map_version: '2'
---
# lang::module_prefix

Selects the module-path rule for the file's language family: Rust module segments, Python module segments, or an empty list for JavaScript-family files.

## Behaviour

- Rust, src/auth/session.rs -> [auth, session]
- Python, pkg/a.py -> [pkg, a]
- JavaScript or TypeScript, any path -> []

## Terms

name prefix, qualified name prefix, module prefix, namespace prefix, scope prefix

## Connections

- **Location:** `crates/myrn-engine/src/indexer/lang/mod.rs:133-139`
- **Signature:** `pub fn module_prefix(lang: Lang, path: &str) -> Vec<String>`
- **In:** [[file-crates-myrn-engine-src-indexer-lang-mod-rs]]
- **Calls:** [[lang-family]], [[lang-rust-module]], [[lang-python-module]]
- **Called by:** [[fileparser-parse]]
