---
id: file-crates-myrn-engine-src-indexer-walk-rs
type: summary
status: unverified
title: crates/myrn-engine/src/indexer/walk.rs
source: myrn summarize
about: file:crates/myrn-engine/src/indexer/walk.rs
anchors:
- file:crates/myrn-engine/src/indexer/walk.rs@b88d9a9358395135
---
# crates/myrn-engine/src/indexer/walk.rs

Repository file walker for the indexer: enumerates supported-language source files under a root, applying .gitignore rules plus an extra ignore file (.myrn/ignore) and a size cap, and returns them as sorted FileStat records. Main parts are the FileStat struct, load_ignore for building the extra matcher, and walk itself.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/walk.rs`
- **In:** [[dir-crates-myrn-engine-src-indexer]]
- **Defines:** [[walk-filestat]], [[walk-load-ignore]], [[walk-walk]]
- **Imports:** [[file-crates-myrn-engine-src-indexer-lang-mod-rs]]
- **Imported by:** [[file-crates-myrn-engine-src-indexer-mod-rs]]
