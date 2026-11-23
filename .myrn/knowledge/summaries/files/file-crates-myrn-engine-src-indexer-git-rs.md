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

Indexes git history into the graph using libgit2: one commit node per commit (message, author, parents, revert target), per-file change rows, and 'touches' edges from each commit to the innermost symbols its hunks changed. Entry points are index_history, which walks back from HEAD, and the HEAD and root-commit lookups it relies on.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/git.rs`
- **In:** [[dir-crates-myrn-engine-src-indexer]]
- **Defines:** [[git-root-commit]], [[git-head-sha]], [[git-index-history]], [[git-index-commit]]
- **Imports:** [[file-crates-myrn-engine-src-indexer-lang-mod-rs]], [[file-crates-myrn-engine-src-indexer-parse-rs]], [[file-crates-myrn-model-src-id-rs]], [[file-crates-myrn-store-src-graph-rs]]
- **Imported by:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-engine-src-indexer-mod-rs]]
