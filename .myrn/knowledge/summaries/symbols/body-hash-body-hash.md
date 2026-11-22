---
id: body-hash-body-hash
type: summary
status: unverified
title: body_hash::body_hash
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/body_hash.rs#indexer::body_hash::body_hash
anchors:
- sym:crates/myrn-engine/src/indexer/body_hash.rs#indexer::body_hash::body_hash@ce1dd8a255a12c13
---
# body_hash::body_hash

Walks the subtree of the given node with a TreeCursor and feeds the bytes of each leaf token, followed by a NUL separator, into blake3, skipping any node whose kind contains 'comment' along with its children. Returns the first 16 hex characters of the digest, so only leaf token text matters and layout does not.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/body_hash.rs:7-31`
- **Signature:** `pub fn body_hash(node: Node<'_>, src: &[u8]) -> String`
- **In:** [[file-crates-myrn-engine-src-indexer-body-hash-rs]]
- **Called by:** [[walker-push-def]]
