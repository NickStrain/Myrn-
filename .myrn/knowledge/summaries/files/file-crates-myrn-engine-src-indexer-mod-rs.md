---
id: file-crates-myrn-engine-src-indexer-mod-rs
type: summary
status: unverified
title: crates/myrn-engine/src/indexer/mod.rs
source: myrn summarize
about: file:crates/myrn-engine/src/indexer/mod.rs
anchors:
- file:crates/myrn-engine/src/indexer/mod.rs@cf2aafb82506fbd9
---
# crates/myrn-engine/src/indexer/mod.rs

Indexer entry point that turns the working tree and Git history into SQLite graph nodes, edges and file rows without a model, re-parsing only files whose content changed. It holds the IndexReport struct, refresh (walk, diff, parse, one transaction, resolve, Git history), and the helpers parse_all, delete_file, persist and gc_modules.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/mod.rs`
- **In:** [[dir-crates-myrn-engine-src-indexer]]
- **Defines:** [[indexer-indexreport]], [[indexreport-changed]], [[indexer-content-hash]], [[indexer-refresh]], [[indexer-parseresult]], [[indexer-parse-all]], [[indexer-delete-file]], [[indexer-persist]], [[indexer-gc-modules]]
- **Imports:** [[file-crates-myrn-engine-src-config-rs]], [[file-crates-myrn-engine-src-indexer-body-hash-rs]], [[file-crates-myrn-engine-src-indexer-git-rs]], [[file-crates-myrn-engine-src-indexer-lang-mod-rs]], [[file-crates-myrn-engine-src-indexer-parse-rs]], [[file-crates-myrn-engine-src-indexer-symbols-rs]], [[file-crates-myrn-engine-src-indexer-walk-rs]], [[file-crates-myrn-model-src-id-rs]], [[file-crates-myrn-store-src-fts-rs]], [[file-crates-myrn-store-src-graph-rs]]
- **Imported by:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-engine-src-lib-rs]]
