---
id: parse-def
type: summary
status: unverified
title: parse::Def
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Def
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Def@45589f786ff0f933
map_version: '2'
---
# parse::Def

Record for one definition (function, method, type, const, test case) with its qualified name, line span, signature, doc, body hash and test flag.

## Behaviour

- qname joins the file module prefix and enclosing scope names with the language separator, so a method in an impl block is Type::method
- A repeated qname in the same file gets #2, #3 suffixes in order of appearance
- module is set only for Rust; is_test is true for Rust test attributes, Python functions named test* in test files or Test* classes, and JS it/test blocks

## Terms

definition, symbol, function record, qualified name, test case, body hash, method, line span

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:14-26`
- **Signature:** `pub struct Def`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
