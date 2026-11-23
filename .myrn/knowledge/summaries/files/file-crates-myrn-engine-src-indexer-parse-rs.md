---
id: file-crates-myrn-engine-src-indexer-parse-rs
type: summary
status: unverified
title: crates/myrn-engine/src/indexer/parse.rs
source: myrn summarize
about: file:crates/myrn-engine/src/indexer/parse.rs
anchors:
- file:crates/myrn-engine/src/indexer/parse.rs@ad1cd2c31aa9da58
---
# crates/myrn-engine/src/indexer/parse.rs

Tree-sitter front end that turns one source file into definitions, call references and imports, with no model and no cross-file resolution. Main parts: FileParser (caches one parser per language and runs the parse), Walker (depth-first AST visitor with a scope stack, local variable types and per-language classifiers for Rust, Python and JS/TS), and helpers for signatures, doc comments, import specs and string unquoting.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs`
- **In:** [[dir-crates-myrn-engine-src-indexer]]
- **Defines:** [[parse-def]], [[parse-refkind]], [[refkind-as-str]], [[refkind-parse]], [[parse-ref]], [[parse-import]], [[parse-parsed]], [[parse-fileparser]], [[fileparser-new]], [[fileparser-parse]], [[parse-scopekind]], [[parse-action]], [[parse-walker]], [[walker-text]], [[walker-field-text]], [[walker-line]], [[walker-rust-mod-path]], [[walker-push-import]], [[walker-push-ref]], [[walker-rust-macro-calls]], [[walker-note-params]], [[walker-note-local]], [[walker-bind-local]], [[walker-in-type-scope]], [[walker-visit]]
- **Imports:** [[file-crates-myrn-engine-src-indexer-body-hash-rs]], [[file-crates-myrn-engine-src-indexer-lang-mod-rs]]
- **Imported by:** [[file-crates-myrn-engine-src-indexer-body-hash-rs]], [[file-crates-myrn-engine-src-indexer-git-rs]], [[file-crates-myrn-engine-src-indexer-mod-rs]], [[file-crates-myrn-engine-src-indexer-symbols-rs]]
