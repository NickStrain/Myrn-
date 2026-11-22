---
id: dir-crates-myrn-model-src
type: summary
status: unverified
title: crates/myrn-model/src/
source: myrn summarize
about: module:crates/myrn-model/src
anchors:
- module:crates/myrn-model/src@bb86275e1220ee6e
---
# crates/myrn-model/src/

This crate is the pure, I/O-free domain model for the Myrn knowledge graph, shared by every agent adapter and the engine. event.rs holds the normalized Event envelope, EventKind, ToolOutcome, and ToolResult that adapters translate their hook payloads into; node.rs defines the Origin (derived vs earned) and 12-variant NodeType taxonomy; relation.rs fixes the closed set of 19 relation names; record.rs defines the Status, EdgeStatus, and Visibility enums; id.rs builds deterministic readable ids for graph nodes, with lib.rs declaring the modules and re-exporting the core types at the crate root.

## Connections

- **In:** [[dir-crates-myrn-model]]
- **Files:** [[file-crates-myrn-model-src-event-rs]], [[file-crates-myrn-model-src-id-rs]], [[file-crates-myrn-model-src-lib-rs]], [[file-crates-myrn-model-src-node-rs]], [[file-crates-myrn-model-src-record-rs]], [[file-crates-myrn-model-src-relation-rs]]
