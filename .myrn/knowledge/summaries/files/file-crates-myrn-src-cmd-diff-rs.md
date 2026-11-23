---
id: file-crates-myrn-src-cmd-diff-rs
type: summary
status: unverified
title: crates/myrn/src/cmd/diff.rs
source: myrn summarize
about: file:crates/myrn/src/cmd/diff.rs
anchors:
- file:crates/myrn/src/cmd/diff.rs@e9b7891b487ac23b
---
# crates/myrn/src/cmd/diff.rs

Implements the myrn diff subcommand, which compares shared knowledge between two points and prints added, removed and changed entries plus a project map summary count. Its only item, run, loads the engine, calls diff_knowledge, and renders text output through ctx.emit.

## Connections

- **Location:** `crates/myrn/src/cmd/diff.rs`
- **In:** [[dir-crates-myrn-src-cmd]]
- **Defines:** [[diff-run]]
- **Imports:** [[file-crates-myrn-src-cmd-mod-rs]]
- **Imported by:** [[file-crates-myrn-src-cmd-mod-rs]]
