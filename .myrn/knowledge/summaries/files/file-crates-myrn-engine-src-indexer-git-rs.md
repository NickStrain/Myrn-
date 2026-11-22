---
id: file-crates-myrn-engine-src-indexer-git-rs
type: summary
status: unverified
title: crates/myrn-engine/src/indexer/git.rs
source: myrn summarize
about: file:crates/myrn-engine/src/indexer/git.rs
anchors:
- file:crates/myrn-engine/src/indexer/git.rs@0c8b66510e3983df
---
# crates/myrn-engine/src/indexer/git.rs

Git history indexer that uses libgit2 directly instead of shelling out. It records each commit as a graph node with its message and metadata, the files it changed (including renames), and 'touches' edges from the commit to the symbols whose line ranges its hunks overlap. index_history drives the walk and index_commit handles a single commit.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/git.rs`
- **In:** [[dir-crates-myrn-engine-src-indexer]]
- **Defines:** [[git-root-commit]], [[git-head-sha]], [[git-index-history]], [[git-index-commit]]
- **Imports:** [[file-crates-myrn-engine-src-indexer-lang-mod-rs]], [[file-crates-myrn-engine-src-indexer-parse-rs]], [[file-crates-myrn-model-src-id-rs]], [[file-crates-myrn-store-src-graph-rs]]
- **Imported by:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-engine-src-indexer-mod-rs]]
