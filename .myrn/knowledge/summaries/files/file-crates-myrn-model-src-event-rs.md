---
id: file-crates-myrn-model-src-event-rs
type: summary
status: unverified
title: crates/myrn-model/src/event.rs
source: myrn summarize
about: file:crates/myrn-model/src/event.rs
anchors:
- file:crates/myrn-model/src/event.rs@0905f5ac3e49d623
---
# crates/myrn-model/src/event.rs

Defines the normalized event model shared by all agent adapters. Each adapter translates its agent's hook payloads into these types, so the engine never handles agent-specific formats. Contains the Event envelope, the EventKind enum, and the ToolOutcome and ToolResult types it uses.

## Connections

- **Location:** `crates/myrn-model/src/event.rs`
- **In:** [[dir-crates-myrn-model-src]]
- **Defines:** [[event-event]], [[event-eventkind]], [[event-tooloutcome]], [[event-toolresult]], [[eventkind-name]]
- **Imported by:** [[file-crates-myrn-adapters-src-adapter-rs]], [[file-crates-myrn-adapters-src-claude-code-rs]], [[file-crates-myrn-adapters-src-codex-rs]], [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-engine-src-capture-mod-rs]], [[file-crates-myrn-engine-tests-records-rs]], [[file-crates-myrn-model-src-lib-rs]], [[file-crates-myrn-src-cmd-hook-rs]]
