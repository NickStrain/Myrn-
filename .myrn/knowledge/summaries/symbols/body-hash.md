---
id: body-hash
type: summary
status: unverified
title: body_hash
source: myrn summarize
about: sym:crates/myrn-engine/tests/notes.rs#body_hash
anchors:
- sym:crates/myrn-engine/tests/notes.rs#body_hash@f314ffa96164ecd9
map_version: '2'
---
# body_hash

Returns the body_hash stored on the node with the given id, and panics if the node or hash is missing.

## Behaviour

- node with a body hash -> that hash string
- unknown id -> panics on unwrap
- node without a body hash -> panics

## Terms

content hash, body digest, node hash, text fingerprint

## Connections

- **Location:** `crates/myrn-engine/tests/notes.rs:108-110`
- **Signature:** `fn body_hash(e: &Engine, id: &str) -> String`
- **In:** [[file-crates-myrn-engine-tests-notes-rs]]
- **Calls:** [[node]]
- **Called by:** [[indexed-repo]]
