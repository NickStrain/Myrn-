---
id: tests-def
type: summary
status: unverified
title: tests::def
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::tests::def
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::tests::def@0b03198bb4831631
map_version: '2'
---
# tests::def

Test helper that returns the Def with the given qualified name, panicking with the known qnames if none matches.

## Behaviour

- def(&p, 'auth::session::refresh') returns that Def
- A missing qname panics, listing every qname in the parse
- Matches the first Def with that qname, so a #2 duplicate must be looked up by its own qname string

## Terms

lookup definition, find def, test lookup, qualified name lookup

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:1295-1302`
- **Signature:** `fn def<'p>(p: &'p Parsed, qname: &str) -> &'p Def`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Called by:** [[walker-classify-rust]], [[walker-classify-js]]
- **Tests:** `rust_defs_refs_imports`, `python_defs_refs_imports`, `typescript_defs_refs_imports_tests`
