---
id: parse-strip-generics
type: summary
status: unverified
title: parse::strip_generics
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::strip_generics
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::strip_generics@834dd07a42cad629
---
# parse::strip_generics

Removes all `<...>` spans from a type string, plus leading '&' and surrounding whitespace.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:1247-1259`
- **Signature:** `fn strip_generics(s: &str) -> String`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Called by:** [[walker-rust-type-name]], [[parse-type-name]]
