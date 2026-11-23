---
id: parse-clean-block-comment
type: summary
status: unverified
title: parse::clean_block_comment
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::clean_block_comment
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::clean_block_comment@695032f47e177d21
map_version: '2'
---
# parse::clean_block_comment

Turns a /** block comment into plain text by removing the opening and closing markers and each line's leading asterisk.

## Behaviour

- /** Issues a session. */ gives 'Issues a session.'
- Each source line becomes one output line, so blank lines stay as empty lines
- Does not check that the comment starts with /**; callers must

## Terms

block comment, JSDoc text, comment markers, doc text, strip comment, asterisk prefix

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:1278-1285`
- **Signature:** `fn clean_block_comment(t: &str) -> String`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Called by:** [[walker-doc]]
