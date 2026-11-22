---
id: parse-def
type: summary
status: unverified
title: parse::Def
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Def
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Def@45589f786ff0f933
---
# parse::Def

Definition record (function, method, type, or test) with qname, kind, line span, signature, doc, body_hash, is_test, and Rust module path; qname gets a '#N' suffix when the same qname repeats in the file.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:14-26`
- **Signature:** `pub struct Def`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
