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

Implements the `myrn rebuild` subcommand, which recreates the engine's database from the source code and notes. Its single function opens the engine at the current directory, runs the rebuild, and prints a summary of the result.

## Connections

- **Location:** `crates/myrn/src/cmd/rebuild.rs`
- **In:** [[dir-crates-myrn-src-cmd]]
- **Defines:** [[rebuild-run]]
- **Imports:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-src-cmd-mod-rs]]
- **Imported by:** [[file-crates-myrn-src-cmd-mod-rs]]
