---
id: file-crates-myrn-model-src-id-rs
type: summary
status: unverified
title: crates/myrn-model/src/id.rs
source: myrn summarize
about: file:crates/myrn-model/src/id.rs
anchors:
- file:crates/myrn-model/src/id.rs@861741214ee86137
---
# crates/myrn-model/src/id.rs

Builds the string identifiers for derived graph nodes (files, modules, symbols, tests, commits, episodes) and provides the parent-directory helper used to derive module ids. Ids are readable and deterministic from path and qualified name; a unit test at the bottom checks symbol, module, and parent_dir behavior.

## Connections

- **Location:** `crates/myrn-model/src/id.rs`
- **In:** [[dir-crates-myrn-model-src]]
- **Defines:** [[id-file-id]], [[id-module-id]], [[id-symbol-id]], [[id-test-id]], [[id-commit-id]], [[id-episode-id]], [[id-parent-dir]]
- **Imported by:** [[file-crates-myrn-engine-src-capture-episode-rs]], [[file-crates-myrn-engine-src-distill-checks-rs]], [[file-crates-myrn-engine-src-indexer-git-rs]], [[file-crates-myrn-engine-src-indexer-mod-rs]], [[file-crates-myrn-engine-src-indexer-symbols-rs]], [[file-crates-myrn-engine-src-summarize-rs]], [[file-crates-myrn-model-src-lib-rs]]
