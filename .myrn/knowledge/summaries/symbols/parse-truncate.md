---
id: parse-truncate
type: summary
status: unverified
title: parse::truncate
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::truncate
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::truncate@334170f6ec9e36f2
map_version: '2'
---
# parse::truncate

Cuts a string to at most max characters (not bytes), replacing the last kept character with an ellipsis when it cuts.

## Behaviour

- ('abc', 5) gives 'abc'; ('abcdef', 4) gives 'abc…'
- Counts Unicode scalar values, so multibyte text is never split mid-character
- max 0 with non-empty input underflows max - 1 and panics in debug builds; all callers pass 80, 200 or 300

## Terms

cap length, shorten, ellipsis, character limit, clip text, trim to length

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:1195-1202`
- **Signature:** `fn truncate(s: &str, max: usize) -> String`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Called by:** [[walker-signature]], [[walker-doc]], [[walker-classify-rust]], [[walker-classify-python]], [[walker-classify-js]]
