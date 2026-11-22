---
id: fileparser-parse
type: summary
status: unverified
title: FileParser::parse
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::FileParser::parse
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::FileParser::parse@bc5abcbcacfff7fa
---
# FileParser::parse

Parses `src` bytes for `lang` and returns Parsed; errors if the grammar cannot be set or tree-sitter returns no tree. `path` is used for the module prefix and for Python test-file detection.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:101-130`
- **Signature:** `pub fn parse(&mut self, lang: Lang, path: &str, src: &[u8]) -> Result<Parsed>`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Calls:** [[lang-grammar]], [[lang-module-prefix]], [[walker-visit]]
- **Called by:** [[tests-hash-of]], [[git-index-commit]], [[indexer-parse-all]], [[tests-parse]]
