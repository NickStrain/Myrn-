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

Defines where Myrn stores its data: the repo's `.myrn/` directory (config, ignore file, shared knowledge) and the per-user data directory (database, notes, episodes, evidence, state), built by `data_home()` and the `Layout` struct. It also provides `repo_rel` for repo-relative, forward-slash paths.

## Connections

- **Location:** `crates/myrn-store/src/paths.rs`
- **In:** [[dir-crates-myrn-store-src]]
- **Defines:** [[paths-data-home]], [[paths-layout]], [[layout-new]], [[layout-config-path]], [[layout-ignore-path]], [[layout-knowledge-dir]], [[layout-state-path]], [[layout-notes-dir]], [[layout-episodes-dir]], [[layout-evidence-dir]], [[layout-create-dirs]], [[paths-repo-rel]]
- **Imported by:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-engine-src-capture-mod-rs]], [[file-crates-myrn-engine-src-distill-record-rs]], [[file-crates-myrn-engine-src-knowledge-rs]], [[file-crates-myrn-store-src-lib-rs]]
