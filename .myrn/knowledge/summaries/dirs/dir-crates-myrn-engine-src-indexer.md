---
id: dir-crates-myrn-engine-src-indexer
type: summary
status: unverified
title: crates/myrn-engine/src/indexer/
source: myrn summarize
about: module:crates/myrn-engine/src/indexer
anchors:
- module:crates/myrn-engine/src/indexer@eaf1937ad25bff01
---
# crates/myrn-engine/src/indexer/

Builds the myrn-engine code graph in SQLite from the working tree and Git history, without a model. mod.rs runs refresh: walk.rs lists source files, parse.rs re-parses only changed ones into definitions, calls and imports, and everything is persisted in one transaction. symbols.rs then resolves cross-file calls, imports and tests edges, and git.rs adds commit nodes and touches edges. lang/ supplies per-language file recognition, grammars, separators and module paths, body_hash.rs gives whitespace- and comment-insensitive hashes, and scip.rs is an empty placeholder for precise references.

## Connections

- **In:** [[dir-crates-myrn-engine-src]]
- **Directories:** [[dir-crates-myrn-engine-src-indexer-lang]]
- **Files:** [[file-crates-myrn-engine-src-indexer-body-hash-rs]], [[file-crates-myrn-engine-src-indexer-git-rs]], [[file-crates-myrn-engine-src-indexer-mod-rs]], [[file-crates-myrn-engine-src-indexer-parse-rs]], [[file-crates-myrn-engine-src-indexer-scip-rs]], [[file-crates-myrn-engine-src-indexer-symbols-rs]], [[file-crates-myrn-engine-src-indexer-walk-rs]]
