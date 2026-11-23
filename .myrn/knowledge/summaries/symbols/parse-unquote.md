---
id: parse-unquote
type: summary
status: unverified
title: parse::unquote
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::unquote
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::unquote@827cffed0f5b5e37
map_version: '2'
---
# parse::unquote

Strips one layer of surrounding quotes (triple, single, double or backtick) and any Python string prefix such as r, b, f or u.

## Behaviour

- A double-quoted ./store gives ./store; r'raw' gives raw; f triple-quoted doc gives doc
- refresh with no quotes is returned unchanged; an empty double-quoted string gives an empty result
- A prefix is removed only when a quote follows it; unmatched quote characters are returned unchanged

## Terms

strip quotes, string literal, string prefix, quoted text, module path string

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:1262-1276`
- **Signature:** `pub fn unquote(s: &str) -> &str`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Called by:** [[walker-doc]], [[walker-classify-js]], [[walker-js-import]], [[walker-js-require]]
- **Tests:** `unquote_strings`
