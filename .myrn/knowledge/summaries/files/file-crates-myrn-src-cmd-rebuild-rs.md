---
id: file-crates-myrn-src-cmd-rebuild-rs
type: summary
status: unverified
title: crates/myrn/src/cmd/rebuild.rs
source: myrn summarize
about: file:crates/myrn/src/cmd/rebuild.rs
anchors:
- file:crates/myrn/src/cmd/rebuild.rs@7c49904ba5278141
---
# crates/myrn/src/cmd/rebuild.rs

Implements the myrn rebuild subcommand. It opens the engine for the current directory, rebuilds the database from the source code and note files (notes are only read), and emits a one-line summary of the counts and elapsed time.

## Connections

- **Location:** `crates/myrn/src/cmd/rebuild.rs`
- **In:** [[dir-crates-myrn-src-cmd]]
- **Defines:** [[rebuild-run]]
- **Imports:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-src-cmd-mod-rs]]
- **Imported by:** [[file-crates-myrn-src-cmd-mod-rs]]
