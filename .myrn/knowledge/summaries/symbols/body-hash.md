---
id: body-hash
type: summary
status: unverified
title: body_hash
source: myrn summarize
about: sym:crates/myrn-engine/tests/notes.rs#body_hash
anchors:
- sym:crates/myrn-engine/tests/notes.rs#body_hash@f314ffa96164ecd9
---
# body_hash

Returns the body_hash of the node with the given id; panics if the node is missing or has no body_hash.

## Connections

- **Location:** `crates/myrn-engine/tests/notes.rs:108-110`
- **Signature:** `fn body_hash(e: &Engine, id: &str) -> String`
- **In:** [[file-crates-myrn-engine-tests-notes-rs]]
- **Calls:** [[node]]
- **Called by:** [[indexed-repo]]
