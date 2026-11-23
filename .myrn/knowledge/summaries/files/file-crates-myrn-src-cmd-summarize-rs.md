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

Implements the myrn summarize command, which writes the project map of summary notes for the project, its directories, files and functions, and asks the model only about code that changed. It builds the engine options from the CLI flags and config, prints a text or JSON report, and sets the exit code from the failure list.

## Connections

- **Location:** `crates/myrn/src/cmd/summarize.rs`
- **In:** [[dir-crates-myrn-src-cmd]]
- **Defines:** [[summarize-run]]
- **Imports:** [[file-crates-myrn-engine-src-summarize-rs]], [[file-crates-myrn-src-cmd-mod-rs]]
- **Imported by:** [[file-crates-myrn-src-cmd-mod-rs]]
