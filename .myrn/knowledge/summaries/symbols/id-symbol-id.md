---
id: id-symbol-id
type: summary
status: unverified
title: id::symbol_id
source: myrn summarize
about: sym:crates/myrn-model/src/id.rs#id::symbol_id
anchors:
- sym:crates/myrn-model/src/id.rs#id::symbol_id@e00e663486962320
---
# id::symbol_id

Takes a path and qualified name and returns `sym:<path>#<qname>`; identity is file plus qualified name, so renaming or moving the symbol changes its id.

## Connections

- **Location:** `crates/myrn-model/src/id.rs:22-24`
- **Signature:** `pub fn symbol_id(path: &str, qname: &str) -> String`
- **In:** [[file-crates-myrn-model-src-id-rs]]
- **Called by:** [[engine-resolve-ref]], [[git-index-commit]], [[indexer-persist]], [[tests-sym]], [[resolver-resolve-sync]]
- **Tests:** `ids`
