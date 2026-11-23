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

Implements the `myrn disconnect` command, which removes hooks, MCP registrations and instruction lines for the named agents, or for every agent in saved state when none are named. It also clears capture consent once no hook-mode agent remains and saves the updated state.

## Connections

- **Location:** `crates/myrn/src/cmd/disconnect.rs`
- **In:** [[dir-crates-myrn-src-cmd]]
- **Defines:** [[disconnect-run]]
- **Imports:** [[file-crates-myrn-src-cmd-connect-rs]], [[file-crates-myrn-src-cmd-mod-rs]]
- **Imported by:** [[file-crates-myrn-src-cmd-mod-rs]]
