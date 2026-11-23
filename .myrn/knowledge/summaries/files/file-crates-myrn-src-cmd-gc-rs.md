---
id: file-crates-myrn-src-cmd-gc-rs
type: summary
status: unverified
title: crates/myrn/src/cmd/gc.rs
source: myrn summarize
about: file:crates/myrn/src/cmd/gc.rs
anchors:
- file:crates/myrn/src/cmd/gc.rs@e364c99322c65d46
---
# crates/myrn/src/cmd/gc.rs

CLI handler for `myrn gc`. It opens the engine, calls `engine.gc` with the age cutoff and dry-run flag, and renders the result report, which covers duplicate note copies, index rows, and session log compaction. The garbage collection logic itself lives in the engine, not in this file.

## Connections

- **Location:** `crates/myrn/src/cmd/gc.rs`
- **In:** [[dir-crates-myrn-src-cmd]]
- **Defines:** [[gc-run]]
- **Imports:** [[file-crates-myrn-src-cmd-mod-rs]]
- **Imported by:** [[file-crates-myrn-src-cmd-mod-rs]]
