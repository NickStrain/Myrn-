---
id: body-hash-body-hash
type: summary
status: unverified
title: body_hash::body_hash
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/body_hash.rs#indexer::body_hash::body_hash
anchors:
- sym:crates/myrn-engine/src/indexer/body_hash.rs#indexer::body_hash::body_hash@ce1dd8a255a12c13
map_version: '2'
---
# body_hash::body_hash

Walks the syntax tree of a node depth-first, feeds each non-comment leaf token's source bytes (each followed by a NUL byte) into blake3, and returns the first 16 hex characters of the digest.

## Behaviour

- "fn f(x: i32) -> i32 { x + 1 }" and the same code with CRLF line breaks and a "// add one" comment produce the same hash
- changing the body from "x + 1" to "x + 2" produces a different hash
- any node whose kind contains "comment" is skipped entirely; whitespace is not hashed because only leaf token bytes are fed in, and NUL separators mean token boundaries matter

## Terms

reformatting, whitespace, comment edits, CRLF vs LF, line endings, content fingerprint, syntax tree hash, normalized hash

## Connections

- **Location:** `crates/myrn-engine/src/indexer/body_hash.rs:7-31`
- **Signature:** `pub fn body_hash(node: Node<'_>, src: &[u8]) -> String`
- **In:** [[file-crates-myrn-engine-src-indexer-body-hash-rs]]
- **Called by:** [[walker-push-def]]
