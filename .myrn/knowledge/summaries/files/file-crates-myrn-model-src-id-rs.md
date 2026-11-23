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

Builds the readable, stable string identifiers used for derived graph nodes (files, modules, symbols, tests, commits, episodes) and includes a helper that extracts a path's parent directory. Each id is a prefix plus the input, so the same code always gets the same id; a unit test at the bottom checks the symbol, root-module, and parent-directory cases.

## Connections

- **Location:** `crates/myrn-model/src/id.rs`
- **In:** [[dir-crates-myrn-model-src]]
- **Defines:** [[id-file-id]], [[id-module-id]], [[id-symbol-id]], [[id-test-id]], [[id-commit-id]], [[id-episode-id]], [[id-parent-dir]]
- **Imported by:** [[file-crates-myrn-engine-src-capture-episode-rs]], [[file-crates-myrn-engine-src-distill-checks-rs]], [[file-crates-myrn-engine-src-indexer-git-rs]], [[file-crates-myrn-engine-src-indexer-mod-rs]], [[file-crates-myrn-engine-src-indexer-symbols-rs]], [[file-crates-myrn-engine-src-summarize-rs]], [[file-crates-myrn-model-src-lib-rs]]
