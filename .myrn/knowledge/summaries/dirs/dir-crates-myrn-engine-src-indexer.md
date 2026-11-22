---
id: dir-crates-myrn-engine-src-indexer
type: summary
status: unverified
title: crates/myrn-engine/src/indexer/
source: myrn summarize
about: module:crates/myrn-engine/src/indexer
anchors:
- module:crates/myrn-engine/src/indexer@686d383e394b27f6
---
# crates/myrn-engine/src/indexer/

Indexer for myrn-engine: builds the code and Git-history graph from the working tree. mod.rs drives refresh, which re-parses only changed files and calls parse.rs (tree-sitter Def/Ref/Import extraction per file, language rules from lang/), walk.rs (gitignore-aware source enumeration), and git.rs (libgit2 commit, file, and symbol-touch edges); symbols.rs then resolves raw references and imports into calls/imports/tests edges across files, while body_hash.rs provides whitespace- and comment-insensitive node hashing for change detection.

## Connections

- **In:** [[dir-crates-myrn-engine-src]]
- **Directories:** [[dir-crates-myrn-engine-src-indexer-lang]]
- **Files:** [[file-crates-myrn-engine-src-indexer-body-hash-rs]], [[file-crates-myrn-engine-src-indexer-git-rs]], [[file-crates-myrn-engine-src-indexer-mod-rs]], [[file-crates-myrn-engine-src-indexer-parse-rs]], [[file-crates-myrn-engine-src-indexer-scip-rs]], [[file-crates-myrn-engine-src-indexer-symbols-rs]], [[file-crates-myrn-engine-src-indexer-walk-rs]]
