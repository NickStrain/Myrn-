---
id: file-crates-myrn-engine-tests-viz-rs
type: summary
status: unverified
title: crates/myrn-engine/tests/viz.rs
source: myrn summarize
about: file:crates/myrn-engine/tests/viz.rs
anchors:
- file:crates/myrn-engine/tests/viz.rs@9da6dfec1da45a5c
---
# crates/myrn-engine/tests/viz.rs

Integration tests for the data behind `myrn ui`: the code graph, notes graph, project map, records, and details. Helpers `write` and `repo` build temporary git fixtures, and the two `#[test]` functions check the Engine's `viz_*` outputs against them.

## Connections

- **Location:** `crates/myrn-engine/tests/viz.rs`
- **In:** [[dir-crates-myrn-engine-tests]]
- **Defines:** [[write-viz]], [[repo-viz]], [[notes-graph-map-records-and-details-find]]
- **Imports:** [[file-crates-myrn-engine-src-api-rs]]
