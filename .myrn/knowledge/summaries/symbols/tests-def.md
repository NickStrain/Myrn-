---
id: tests-def
type: summary
status: unverified
title: tests::def
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::tests::def
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::tests::def@0b03198bb4831631
---
# tests::def

Test helper: finds a Def by qname, panicking with the available qnames if missing.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:1295-1302`
- **Signature:** `fn def<'p>(p: &'p Parsed, qname: &str) -> &'p Def`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Called by:** [[walker-classify-rust]], [[walker-classify-js]]
- **Tests:** `rust_defs_refs_imports`, `python_defs_refs_imports`, `typescript_defs_refs_imports_tests`
