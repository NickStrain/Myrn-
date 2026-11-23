---
id: parse-fileparser
type: summary
status: unverified
title: parse::FileParser
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::FileParser
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::FileParser@3191322d5330d37f
map_version: '2'
---
# parse::FileParser

Holds one tree-sitter Parser per language in a map so repeated parses reuse it.

## Behaviour

- Parsers are created lazily on the first parse for each Lang
- Reuse one FileParser across files to avoid rebuilding grammars
- Default gives an empty map, same as new()

## Terms

parser cache, parser pool, language parsers, tree-sitter handle, grammar cache

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:92-94`
- **Signature:** `pub struct FileParser`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
