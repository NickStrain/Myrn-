---
id: file-crates-myrn-store-src-paths-rs
type: summary
status: unverified
title: crates/myrn-store/src/paths.rs
source: myrn summarize
about: file:crates/myrn-store/src/paths.rs
anchors:
- file:crates/myrn-store/src/paths.rs@a6f3f19eb2b4c8d3
---
# crates/myrn-store/src/paths.rs

Resolves Myrn's storage locations: the repo-local .myrn directory (project.toml, ignore, knowledge/) and the per-user private data directory. Defines the Layout struct that derives every per-project path and creates the directory tree, plus repo_rel for forward-slash repo-relative paths.

## Connections

- **Location:** `crates/myrn-store/src/paths.rs`
- **In:** [[dir-crates-myrn-store-src]]
- **Defines:** [[paths-data-home]], [[paths-layout]], [[layout-new]], [[layout-config-path]], [[layout-ignore-path]], [[layout-knowledge-dir]], [[layout-state-path]], [[layout-notes-dir]], [[layout-episodes-dir]], [[layout-evidence-dir]], [[layout-create-dirs]], [[paths-repo-rel]]
- **Imported by:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-engine-src-capture-mod-rs]], [[file-crates-myrn-engine-src-distill-record-rs]], [[file-crates-myrn-store-src-lib-rs]]
