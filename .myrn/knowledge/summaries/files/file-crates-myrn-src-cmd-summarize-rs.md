---
id: file-crates-myrn-src-cmd-summarize-rs
type: summary
status: unverified
title: crates/myrn/src/cmd/summarize.rs
source: myrn summarize
about: file:crates/myrn/src/cmd/summarize.rs
anchors:
- file:crates/myrn/src/cmd/summarize.rs@799ffc1463a4ca8b
---
# crates/myrn/src/cmd/summarize.rs

CLI handler for `myrn summarize`, which writes the project map of summary notes (project, directories, files, functions and types) into the Obsidian vault. It merges command-line flags with the `summarize` config, calls the engine, and prints either a dry-run staleness count or a full report. Progress lines go to stderr unless JSON output is requested.

## Connections

- **Location:** `crates/myrn/src/cmd/summarize.rs`
- **In:** [[dir-crates-myrn-src-cmd]]
- **Defines:** [[summarize-run]]
- **Imports:** [[file-crates-myrn-engine-src-summarize-rs]], [[file-crates-myrn-src-cmd-mod-rs]]
- **Imported by:** [[file-crates-myrn-src-cmd-mod-rs]]
