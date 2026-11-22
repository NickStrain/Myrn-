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

Tree-sitter front end for the indexer: parses one Rust, Python, or JS/TS source file into Def, Ref, and Import records via a Walker that classifies nodes per language family, tracks scopes and locals for method-to-path rewriting, and extracts docs and signatures. Nothing is resolved across files here; FileParser::parse is the only entry point and owns per-language parser caching.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs`
- **In:** [[dir-crates-myrn-engine-src-indexer]]
- **Defines:** [[parse-def]], [[parse-refkind]], [[refkind-as-str]], [[refkind-parse]], [[parse-ref]], [[parse-import]], [[parse-parsed]], [[parse-fileparser]], [[fileparser-new]], [[fileparser-parse]], [[parse-scopekind]], [[parse-action]], [[parse-walker]], [[walker-text]], [[walker-field-text]], [[walker-line]], [[walker-rust-mod-path]], [[walker-push-import]], [[walker-push-ref]], [[walker-rust-macro-calls]], [[walker-note-params]], [[walker-note-local]], [[walker-bind-local]], [[walker-in-type-scope]], [[walker-visit]]
- **Imports:** [[file-crates-myrn-engine-src-indexer-body-hash-rs]], [[file-crates-myrn-engine-src-indexer-lang-mod-rs]]
- **Imported by:** [[file-crates-myrn-engine-src-indexer-body-hash-rs]], [[file-crates-myrn-engine-src-indexer-git-rs]], [[file-crates-myrn-engine-src-indexer-mod-rs]], [[file-crates-myrn-engine-src-indexer-symbols-rs]]
