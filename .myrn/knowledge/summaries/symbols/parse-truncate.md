---
id: parse-truncate
type: summary
status: unverified
title: parse::truncate
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::truncate
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::truncate@334170f6ec9e36f2
---
# parse::truncate

Truncates to at most `max` characters (not bytes), replacing the last with an ellipsis when cut.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:1195-1202`
- **Signature:** `fn truncate(s: &str, max: usize) -> String`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Called by:** [[walker-signature]], [[walker-doc]], [[walker-classify-rust]], [[walker-classify-python]], [[walker-classify-js]]
