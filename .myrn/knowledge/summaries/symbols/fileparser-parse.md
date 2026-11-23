---
id: fileparser-parse
type: summary
status: unverified
title: FileParser::parse
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::FileParser::parse
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::FileParser::parse@bc5abcbcacfff7fa
map_version: '2'
---
# FileParser::parse

Parses source bytes for one language and walks the tree to return definitions, references, imports and the error flag.

## Behaviour

- Returns Err if setting the grammar fails or tree-sitter yields no tree; a tree with syntax errors still returns Ok with has_errors true
- path sets the Rust module prefix (via lang::module_prefix) and Python test-file detection; it is not read from disk
- The walker stops descending past depth 512, so deeper nodes are silently skipped

## Terms

parse file, index file, read source, walk tree, extract symbols, syntax error, module prefix, tree walk

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:101-130`
- **Signature:** `pub fn parse(&mut self, lang: Lang, path: &str, src: &[u8]) -> Result<Parsed>`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Calls:** [[lang-grammar]], [[lang-module-prefix]], [[walker-visit]]
- **Called by:** [[tests-hash-of]], [[git-index-commit]], [[indexer-parse-all]], [[tests-parse]]
