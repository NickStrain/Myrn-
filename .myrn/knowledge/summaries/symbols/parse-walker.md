---
id: parse-walker
type: summary
status: unverified
title: parse::Walker
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker@fb9eb6ef5301d8a4
map_version: '2'
---
# parse::Walker

Per-file visitor state: language, file path, module prefix, scope stack, current enclosing definition, seen qualified names, local variable types and the output Parsed.

## Behaviour

- Created fresh inside each FileParser::parse call and discarded afterwards
- locals holds variable types only for the current definition; visit restores the previous set when leaving it
- seen counts each qname so duplicates get #n suffixes

## Terms

AST walker, visitor state, traversal state, tree walker, parse context

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:163-174`
- **Signature:** `struct Walker<'a>`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
