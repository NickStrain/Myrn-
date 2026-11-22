---
id: file-crates-myrn-store-src-state-rs
type: summary
status: unverified
title: crates/myrn-store/src/state.rs
source: myrn summarize
about: file:crates/myrn-store/src/state.rs
anchors:
- file:crates/myrn-store/src/state.rs@dd27e3070747f1e6
---
# crates/myrn-store/src/state.rs

Private per-project state, stored as state.json next to memory.db so it survives `myrn rebuild` and never goes in the repo. It holds the recording consent flag, connected agents, and a capped hook-timing log, with load and save helpers.

## Connections

- **Location:** `crates/myrn-store/src/state.rs`
- **In:** [[dir-crates-myrn-store-src]]
- **Defines:** [[state-state]], [[state-agentstate]], [[state-hooktiming]], [[state-load]], [[state-save]], [[state-record-timing]]
- **Imported by:** [[file-crates-myrn-store-src-lib-rs]], [[file-crates-myrn-src-cmd-connect-rs]], [[file-crates-myrn-src-cmd-hook-rs]]
