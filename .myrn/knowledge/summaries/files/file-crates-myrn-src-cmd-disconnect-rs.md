---
id: file-crates-myrn-src-cmd-disconnect-rs
type: summary
status: unverified
title: crates/myrn/src/cmd/disconnect.rs
source: myrn summarize
about: file:crates/myrn/src/cmd/disconnect.rs
anchors:
- file:crates/myrn/src/cmd/disconnect.rs@85a1fceff8d7e784
---
# crates/myrn/src/cmd/disconnect.rs

Implements the `myrn disconnect` subcommand, which removes the hooks, MCP registrations, and instruction lines that were installed for connected agents, and updates the saved engine state to match. Its only item is the `run` function.

## Connections

- **Location:** `crates/myrn/src/cmd/disconnect.rs`
- **In:** [[dir-crates-myrn-src-cmd]]
- **Defines:** [[disconnect-run]]
- **Imports:** [[file-crates-myrn-src-cmd-connect-rs]], [[file-crates-myrn-src-cmd-mod-rs]]
- **Imported by:** [[file-crates-myrn-src-cmd-mod-rs]]
