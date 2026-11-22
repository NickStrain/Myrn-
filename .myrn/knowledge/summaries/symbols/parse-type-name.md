---
id: parse-type-name
type: summary
status: unverified
title: parse::type_name
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::type_name
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::type_name@96a5ef56236be2e5
---
# parse::type_name

Parses a type annotation to a bare nominal name, dropping &, mut, dyn, generics, and module paths; returns None for lowercase names, unions, and common wrappers like Option, Vec, Box, Rc, Arc, and Dict.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:1217-1244`
- **Signature:** `fn type_name(raw: &str) -> Option<String>`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Calls:** [[parse-strip-generics]]
- **Called by:** [[walker-bind-local]]
