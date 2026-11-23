---
id: file-crates-myrn-src-cmd-init-rs
type: summary
status: unverified
title: crates/myrn/src/cmd/init.rs
source: myrn summarize
about: file:crates/myrn/src/cmd/init.rs
anchors:
- file:crates/myrn/src/cmd/init.rs@11c64beb0b5a5ab8
---
# crates/myrn/src/cmd/init.rs

CLI entry point for the `myrn init` subcommand. It indexes the current working directory through the engine, creates the local store, and prints a plain-text summary of the indexing counts, link counts, warnings, and storage locations. It records no session data.

## Connections

- **Location:** `crates/myrn/src/cmd/init.rs`
- **In:** [[dir-crates-myrn-src-cmd]]
- **Defines:** [[init-run]]
- **Imports:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-src-cmd-mod-rs]]
- **Imported by:** [[file-crates-myrn-src-cmd-mod-rs]]
