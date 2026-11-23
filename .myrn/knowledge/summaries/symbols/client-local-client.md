---
id: client-local-client
type: summary
status: unverified
title: client::local
source: myrn summarize
about: sym:crates/myrn-embed/src/client.rs#client::local#2
anchors:
- sym:crates/myrn-embed/src/client.rs#client::local#2@c11498aa3cba2e06
map_version: '2'
---
# client::local

Stub used when the model feature is disabled; it ignores its arguments and always returns None.

## Behaviour

- any inputs -> None
- texts are never encoded
- data_home is unused

## Terms

no model feature, stub, disabled, always none, feature off, compiled out

## Connections

- **Location:** `crates/myrn-embed/src/client.rs:92-94`
- **Signature:** `pub fn local(_data_home: &Path, _texts: &[String]) -> Option<Vec<Vec<f32>>>`
- **In:** [[file-crates-myrn-embed-src-client-rs]]
- **Called by:** [[client-embed]]
