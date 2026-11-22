---
id: id-test-id
type: summary
status: unverified
title: id::test_id
source: myrn summarize
about: sym:crates/myrn-model/src/id.rs#id::test_id
anchors:
- sym:crates/myrn-model/src/id.rs#id::test_id@2926a97da95b6013
---
# id::test_id

Takes a path and qualified name and returns `test:<path>#<qname>`, using the same scheme as symbol_id but with a `test:` prefix.

## Connections

- **Location:** `crates/myrn-model/src/id.rs:27-29`
- **Signature:** `pub fn test_id(path: &str, qname: &str) -> String`
- **In:** [[file-crates-myrn-model-src-id-rs]]
- **Called by:** [[engine-resolve-ref]], [[git-index-commit]], [[indexer-persist]], [[resolver-resolve-sync]]
