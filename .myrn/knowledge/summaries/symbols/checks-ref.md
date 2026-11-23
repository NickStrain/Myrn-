---
id: checks-ref
type: summary
status: unverified
title: checks::Ref
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/checks.rs#distill::checks::Ref
anchors:
- sym:crates/myrn-engine/src/distill/checks.rs#distill::checks::Ref@a2287516f09af765
map_version: '2'
---
# checks::Ref

A resolved reference: the node id, its node type, the header text the note will show, and an optional anchor holding the body hash.

## Behaviour

- header is qname, path#qname, a path, a 12-char short sha, or a note id
- anchor is set only for symbol and test nodes that have a body_hash
- commit and note refs always have anchor None

## Terms

resolved reference, header, anchor, body hash, node id, note header

## Connections

- **Location:** `crates/myrn-engine/src/distill/checks.rs:22-30`
- **Signature:** `pub struct Ref`
- **In:** [[file-crates-myrn-engine-src-distill-checks-rs]]
