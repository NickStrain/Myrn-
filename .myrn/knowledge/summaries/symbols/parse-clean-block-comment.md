---
id: parse-clean-block-comment
type: summary
status: unverified
title: parse::clean_block_comment
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::clean_block_comment
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::clean_block_comment@695032f47e177d21
---
# parse::clean_block_comment

Turns a /** */ comment into plain text: removes the delimiters, leading '*' per line, and trims each line.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:1278-1285`
- **Signature:** `fn clean_block_comment(t: &str) -> String`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Called by:** [[walker-doc]]
