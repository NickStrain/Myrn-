---
id: review-coderef
type: summary
status: unverified
title: review::CodeRef
source: myrn summarize
about: sym:crates/myrn-engine/src/review.rs#review::CodeRef
anchors:
- sym:crates/myrn-engine/src/review.rs#review::CodeRef@f1887cd9d8ce06f3
map_version: '2'
---
# review::CodeRef

One code location a record points at (name, location, language, excerpt text) and whether that code changed since the record was made.

## Behaviour

- changed is true when the anchor stored a body hash and the node's current hash does not start with it, or is missing
- a linked-only target has no anchor hash, so changed is false
- unreadable file -> source is an empty string

## Terms

code anchor, stale anchor, drifted code, body hash, code excerpt, changed since recorded, code reference

## Connections

- **Location:** `crates/myrn-engine/src/review.rs:38-45`
- **Signature:** `pub struct CodeRef`
- **In:** [[file-crates-myrn-engine-src-review-rs]]
