---
id: fileparser-new
type: summary
status: unverified
title: FileParser::new
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::FileParser::new
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::FileParser::new@1febdd0f8610528b
map_version: '2'
---
# FileParser::new

Creates an empty FileParser with no grammars loaded yet.

## Behaviour

- A new FileParser has no parsers; the first parse for a language loads its grammar
- Each language gets its own parser, created on first use

## Terms

constructor, create parser, initialise, empty cache, fresh parser

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:97-99`
- **Signature:** `pub fn new() -> FileParser`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Called by:** [[tests-hash-of]], [[git-index-history]], [[indexer-parse-all]], [[tests-parse]]
