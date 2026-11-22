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

Implements the myrn connect command. It prints the session-recording disclosure, gets consent before installing hooks, chooses which agent adapters to install, runs their install step, and records per-agent install state. Helpers select adapters, prompt on stdin, and produce timestamps.

## Connections

- **Location:** `crates/myrn/src/cmd/connect.rs`
- **In:** [[dir-crates-myrn-src-cmd]]
- **Defines:** [[connect-args]], [[connect-install-ctx]], [[connect-choose]], [[connect-ask]], [[connect-run]], [[connect-now]]
- **Imports:** [[file-crates-myrn-adapters-src-lib-rs]], [[file-crates-myrn-store-src-state-rs]], [[file-crates-myrn-src-cmd-mod-rs]]
- **Imported by:** [[file-crates-myrn-src-cmd-disconnect-rs]], [[file-crates-myrn-src-cmd-doctor-rs]], [[file-crates-myrn-src-cmd-mod-rs]]
