---
id: id-symbol-id
type: summary
status: unverified
title: id::symbol_id
source: myrn summarize
about: sym:crates/myrn-model/src/id.rs#id::symbol_id
anchors:
- sym:crates/myrn-model/src/id.rs#id::symbol_id@e00e663486962320
map_version: '2'
---
# id::symbol_id

Builds a symbol node id from a file path and a qualified name, joined with `#`.

## Behaviour

- ("src/a.rs", "a::f") -> "sym:src/a.rs#a::f"
- Splits on the first `#` only when parsed by a consumer; the function itself does not escape or check for `#` in the inputs

## Terms

function id, definition id, qualified name, symbol key, method id

## Connections

- **Location:** `crates/myrn-model/src/id.rs:22-24`
- **Signature:** `pub fn symbol_id(path: &str, qname: &str) -> String`
- **In:** [[file-crates-myrn-model-src-id-rs]]
- **Called by:** [[engine-resolve-ref]], [[git-index-commit]], [[indexer-persist]], [[tests-sym]], [[resolver-resolve-sync]]
- **Tests:** `ids`
