---
id: file-crates-myrn-src-cmd-connect-rs
type: summary
status: unverified
title: crates/myrn/src/cmd/connect.rs
source: myrn summarize
about: file:crates/myrn/src/cmd/connect.rs
anchors:
- file:crates/myrn/src/cmd/connect.rs@fe566c092b8654dd
---
# crates/myrn/src/cmd/connect.rs

The `myrn connect` subcommand: shows the session-recording disclosure, asks for consent, picks which agent adapters to install, and writes their setup (hooks or pull-only MCP) and the per-agent state. Main parts are the `Args` struct, adapter selection in `choose`, the consent prompt in `ask`, the `run` flow, and the `install_ctx` and `now` helpers.

## Connections

- **Location:** `crates/myrn/src/cmd/connect.rs`
- **In:** [[dir-crates-myrn-src-cmd]]
- **Defines:** [[connect-args]], [[connect-install-ctx]], [[connect-choose]], [[connect-ask]], [[connect-run]], [[connect-now]]
- **Imports:** [[file-crates-myrn-adapters-src-lib-rs]], [[file-crates-myrn-store-src-state-rs]], [[file-crates-myrn-src-cmd-mod-rs]]
- **Imported by:** [[file-crates-myrn-src-cmd-disconnect-rs]], [[file-crates-myrn-src-cmd-doctor-rs]], [[file-crates-myrn-src-cmd-mod-rs]]
