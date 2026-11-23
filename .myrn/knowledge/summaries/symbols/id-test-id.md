---
id: id-test-id
type: summary
status: unverified
title: id::test_id
source: myrn summarize
about: sym:crates/myrn-model/src/id.rs#id::test_id
anchors:
- sym:crates/myrn-model/src/id.rs#id::test_id@2926a97da95b6013
map_version: '2'
---
# id::test_id

Builds a test node id from a file path and a qualified name, using the `test:` prefix instead of `sym:`.

## Behaviour

- ("tests/a.rs", "a::t") -> "test:tests/a.rs#a::t"
- Same format as symbol_id, so only the prefix distinguishes a test from a symbol

## Terms

test node, test case id, unit test, test identifier

## Connections

- **Location:** `crates/myrn-model/src/id.rs:27-29`
- **Signature:** `pub fn test_id(path: &str, qname: &str) -> String`
- **In:** [[file-crates-myrn-model-src-id-rs]]
- **Called by:** [[engine-resolve-ref]], [[git-index-commit]], [[indexer-persist]], [[resolver-resolve-sync]]
