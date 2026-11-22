---
id: file-crates-myrn-src-cmd-hook-rs
type: summary
status: unverified
title: crates/myrn/src/cmd/hook.rs
source: myrn summarize
about: file:crates/myrn/src/cmd/hook.rs
anchors:
- file:crates/myrn/src/cmd/hook.rs@3526fe931a425d37
---
# crates/myrn/src/cmd/hook.rs

Implements the 'myrn hook <agent> <event>' subcommand that agent hook scripts call. It reads the agent's event payload from stdin, maps it through the agent adapter, writes any context to inject on stdout, records the event, and saves hook timing; it holds the never-fail contract, the 4 MiB stdin cap (MAX_PAYLOAD), and the entry points run and handle.

## Connections

- **Location:** `crates/myrn/src/cmd/hook.rs`
- **In:** [[dir-crates-myrn-src-cmd]]
- **Defines:** [[hook-run]], [[hook-handle]]
- **Imports:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-model-src-event-rs]], [[file-crates-myrn-store-src-state-rs]], [[file-crates-myrn-src-cmd-mod-rs]]
- **Imported by:** [[file-crates-myrn-src-cmd-mod-rs]]
