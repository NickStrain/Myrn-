---
id: parse-ref
type: summary
status: unverified
title: parse::Ref
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Ref
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Ref@a86d6c3300487abb
map_version: '2'
---
# parse::Ref

One call site inside a definition, with the callee name, optional qualifier (receiver or path prefix) and line.

## Behaviour

- from is an index into Parsed.defs of the innermost enclosing definition
- Calls outside any definition, or with an empty name, are never stored
- qualifier is the receiver text for Method and the path text for Path; None for plain Call

## Terms

call site, call reference, caller, callee, call edge, invocation

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:58-65`
- **Signature:** `pub struct Ref`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
