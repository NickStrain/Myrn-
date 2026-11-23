---
id: parse-parsed
type: summary
status: unverified
title: parse::Parsed
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Parsed
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Parsed@ee9c5068e08c4183
map_version: '2'
---
# parse::Parsed

Result of parsing one file: definitions, call references, imports and a flag for syntax errors.

## Behaviour

- Default gives empty vectors and has_errors false
- has_errors is root.has_error(); partial results are still returned rather than an error
- A file tree-sitter cannot parse at all makes FileParser::parse return Err instead

## Terms

parse result, file result, syntax error flag, parse output, partial index

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:82-88`
- **Signature:** `pub struct Parsed`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
