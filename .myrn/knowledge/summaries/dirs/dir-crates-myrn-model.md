---
id: dir-crates-myrn-model
type: summary
status: unverified
title: crates/myrn-model/
source: myrn summarize
about: module:crates/myrn-model
anchors:
- module:crates/myrn-model@baffaf6e9c1b0712
---
# crates/myrn-model/

Pure, I/O-free domain model for the Myrn knowledge graph, with no side effects or filesystem access. lib.rs exposes the event, id, node, record, and relation modules: node.rs defines the 12 NodeType kinds and their derived or earned Origin, record.rs holds the Status, EdgeStatus, and Visibility metadata, relation.rs fixes the closed set of 19 Relation names, id.rs builds stable prefixed graph node ids, and event.rs defines the normalized Event types that adapters translate hook payloads into.

## Connections

- **In:** [[dir-crates]]
- **Directories:** [[dir-crates-myrn-model-src]]
