---
id: file-crates-myrn-src-cmd-forget-rs
type: summary
status: unverified
title: crates/myrn/src/cmd/forget.rs
source: myrn summarize
about: file:crates/myrn/src/cmd/forget.rs
anchors:
- file:crates/myrn/src/cmd/forget.rs@801ed9d7c8eecb08
---
# crates/myrn/src/cmd/forget.rs

Implements the `myrn forget <id>` command, which permanently removes a record and anything derived only from it. It plans the removal with the engine, asks for y/N confirmation on a terminal unless `yes` is set, then performs the removal and prints the result.

## Connections

- **Location:** `crates/myrn/src/cmd/forget.rs`
- **In:** [[dir-crates-myrn-src-cmd]]
- **Defines:** [[forget-run]]
- **Imports:** [[file-crates-myrn-src-cmd-mod-rs]]
- **Imported by:** [[file-crates-myrn-src-cmd-mod-rs]]
