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

Repository file walker for the indexer: it lists supported-language source files under a root, honouring standard .gitignore rules plus an extra ignore file (.myrn/ignore). Main parts are the FileStat record, load_ignore (builds the extra matcher), and walk (traverses, filters, and returns sorted FileStat entries).

## Connections

- **Location:** `crates/myrn-engine/src/indexer/walk.rs`
- **In:** [[dir-crates-myrn-engine-src-indexer]]
- **Defines:** [[walk-filestat]], [[walk-load-ignore]], [[walk-walk]]
- **Imports:** [[file-crates-myrn-engine-src-indexer-lang-mod-rs]]
- **Imported by:** [[file-crates-myrn-engine-src-indexer-mod-rs]]
