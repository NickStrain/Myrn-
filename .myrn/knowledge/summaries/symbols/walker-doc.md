---
id: walker-doc
type: summary
status: unverified
title: Walker::doc
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::doc
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::doc@9621059d3fc2d401
map_version: '2'
---
# Walker::doc

Pulls the first paragraph of a definition's documentation: a Python docstring, Rust /// or /** */ lines above it, or a JSDoc block above it.

## Behaviour

- Rust '/// a', '///', '/// b' gives 'a'; '////' lines are not doc comments
- Python first statement '"""A login session."""' gives 'A login session.'
- A plain // comment above a JS function gives None, since only /** blocks count; results are whitespace-collapsed, capped at 300 characters, and empty gives None

## Terms

docstring, doc comment, documentation, summary, JSDoc, rustdoc, first paragraph

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:529-587`
- **Signature:** `fn doc(&self, node: Node<'_>, anchor: Option<Node<'_>>) -> Option<String>`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Calls:** [[lang-family]], [[walker-text]], [[parse-truncate]], [[parse-unquote]], [[parse-clean-block-comment]], [[sym-parent]]
- **Called by:** [[walker-push-def]]
