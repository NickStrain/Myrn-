---
id: file-crates-myrn-src-cmd-status-rs
type: summary
status: unverified
title: crates/myrn/src/cmd/status.rs
source: myrn summarize
about: file:crates/myrn/src/cmd/status.rs
anchors:
- file:crates/myrn/src/cmd/status.rs@20589ca591cb175f
---
# crates/myrn/src/cmd/status.rs

Implements the `myrn status` CLI subcommand: it reads engine status and prints a plain-text report covering project id, file counts by language, graph and link counts, record states, note health, connected agents, and the store path. Output is produced via `ctx.emit`, which serializes the status struct for JSON output or runs the text renderer otherwise.

## Connections

- **Location:** `crates/myrn/src/cmd/status.rs`
- **In:** [[dir-crates-myrn-src-cmd]]
- **Defines:** [[status-run]]
- **Imports:** [[file-crates-myrn-src-cmd-mod-rs]]
- **Imported by:** [[file-crates-myrn-src-cmd-mod-rs]]
