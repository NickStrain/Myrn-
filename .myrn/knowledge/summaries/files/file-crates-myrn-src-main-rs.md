---
id: file-crates-myrn-src-main-rs
type: summary
status: unverified
title: crates/myrn/src/main.rs
source: myrn summarize
about: file:crates/myrn/src/main.rs
anchors:
- file:crates/myrn/src/main.rs@0362e113a7a420ed
---
# crates/myrn/src/main.rs

Entry point for the `myrn` binary: it defines the clap CLI structs and enums (global flags, every subcommand, embeddings and debug subcommands), then `main` parses arguments, builds `cmd::Ctx`, and dispatches each command to its `cmd::*` handler. Errors go to stderr as `myrn: ...` with a failure exit code.

## Connections

- **Location:** `crates/myrn/src/main.rs`
- **In:** [[dir-crates-myrn-src]]
- **Defines:** [[cli-main]], [[command]], [[embeddingsaction]], [[debugcommand]], [[main-main]]
- **Imports:** [[file-crates-myrn-src-cmd-mod-rs]]
