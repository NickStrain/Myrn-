---
id: dir-crates-myrn-model-src
type: summary
status: unverified
title: crates/myrn-model/src/
source: myrn summarize
about: module:crates/myrn-model/src
anchors:
- module:crates/myrn-model/src@aa4f3e8ec6bc651c
---
# crates/myrn-model/src/

Pure, I/O-free domain model for the Myrn knowledge graph. lib.rs declares the event, id, node, record, and relation modules. node.rs defines the 12 NodeType kinds and their derived or earned Origin, record.rs holds the Status, EdgeStatus, and Visibility metadata, relation.rs fixes the closed set of 19 Relation names, id.rs builds stable prefixed ids for graph nodes, and event.rs defines the normalized Event types that adapters translate their hook payloads into.

## Connections

- **In:** [[dir-crates-myrn-model]]
- **Files:** [[file-crates-myrn-model-src-event-rs]], [[file-crates-myrn-model-src-id-rs]], [[file-crates-myrn-model-src-lib-rs]], [[file-crates-myrn-model-src-node-rs]], [[file-crates-myrn-model-src-record-rs]], [[file-crates-myrn-model-src-relation-rs]]
