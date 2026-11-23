---
id: parse-refkind
type: summary
status: unverified
title: parse::RefKind
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::RefKind
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::RefKind@a9b9a4b32dbdfd68
map_version: '2'
---
# parse::RefKind

Kind of a call reference: plain call, path-qualified call, or method call on a receiver.

## Behaviour

- f() -> Call
- Type::f() or module::f() -> Path
- recv.f() -> Method, rewritten to Path when recv has a known local type

## Terms

call form, call category, call type, reference type, qualified call, receiver call, dot call

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:29-36`
- **Signature:** `pub enum RefKind`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
