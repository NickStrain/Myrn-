---
id: lang-grammar
type: summary
status: unverified
title: Lang::grammar
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/lang/mod.rs#indexer::lang::Lang::grammar
anchors:
- sym:crates/myrn-engine/src/indexer/lang/mod.rs#indexer::lang::Lang::grammar@c2b24410df892ad3
---
# Lang::grammar

Returns the tree-sitter Language for the variant. TypeScript and Tsx use different grammars from the tree_sitter_typescript crate.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/lang/mod.rs:55-63`
- **Signature:** `pub fn grammar(self) -> Language`
- **In:** [[file-crates-myrn-engine-src-indexer-lang-mod-rs]]
- **Called by:** [[engine-doctor]], [[fileparser-parse]]
