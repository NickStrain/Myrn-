---
id: parse-unquote
type: summary
status: unverified
title: parse::unquote
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::unquote
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::unquote@827cffed0f5b5e37
---
# parse::unquote

Strips a string literal's quotes (single, double, triple, or backtick) and any leading Python prefix (r, b, f, u in any case) when present; returns the input unchanged otherwise.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:1262-1276`
- **Signature:** `pub fn unquote(s: &str) -> &str`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Called by:** [[walker-doc]], [[walker-classify-js]], [[walker-js-import]], [[walker-js-require]]
- **Tests:** `unquote_strings`
