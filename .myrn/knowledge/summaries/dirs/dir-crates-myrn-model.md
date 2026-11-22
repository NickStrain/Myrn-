---
id: dir-crates-myrn-model
type: summary
status: unverified
title: crates/myrn-model/
source: myrn summarize
about: module:crates/myrn-model
anchors:
- module:crates/myrn-model@5b131ad6c682536d
---
# crates/myrn-model/

Pure, I/O-free domain model for the Myrn knowledge graph, shared by all agent adapters and the engine; it has no file, network, or process access. event.rs defines the normalized Event envelope with EventKind, ToolOutcome, and ToolResult that adapters translate hook payloads into, node.rs defines Origin (derived vs earned) and the 12-variant NodeType, relation.rs fixes the closed set of 19 relation names, record.rs defines Status, EdgeStatus, and Visibility, and id.rs builds deterministic readable node ids, with lib.rs declaring the modules and re-exporting the core types at the crate root.

## Connections

- **In:** [[dir-crates]]
- **Directories:** [[dir-crates-myrn-model-src]]
