---
id: file-crates-myrn-src-cmd-hook-rs
type: summary
status: unverified
title: crates/myrn/src/cmd/hook.rs
source: myrn summarize
about: file:crates/myrn/src/cmd/hook.rs
anchors:
- file:crates/myrn/src/cmd/hook.rs@9b6232a4a22d2af1
---
# crates/myrn/src/cmd/hook.rs

Entry point for `myrn hook <agent> <event>`, which agent hooks call with a JSON payload on stdin. Contains `run`, which prints the result and always exits 0, and `handle`, which translates the payload, injects or recalls context, records usage, outcome, and timing, and returns the adapter-formatted output.

## Connections

- **Location:** `crates/myrn/src/cmd/hook.rs`
- **In:** [[dir-crates-myrn-src-cmd]]
- **Defines:** [[hook-run]], [[hook-handle]]
- **Imports:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-model-src-event-rs]], [[file-crates-myrn-store-src-state-rs]], [[file-crates-myrn-src-cmd-mod-rs]]
- **Imported by:** [[file-crates-myrn-src-cmd-mod-rs]]
