---
id: node-records
type: summary
status: unverified
title: node
source: myrn summarize
about: sym:crates/myrn-engine/tests/records.rs#node
anchors:
- sym:crates/myrn-engine/tests/records.rs#node@983687af1d3fd2a6
---
# node

Fetches the graph node with the given id from the database and panics if it is missing.

## Connections

- **Location:** `crates/myrn-engine/tests/records.rs:130-132`
- **Signature:** `fn node(e: &Engine, id: &str) -> graph::Node`
- **In:** [[file-crates-myrn-engine-tests-records-rs]]
- **Calls:** [[engine-db]], [[graph-node-graph]]
- **Tests:** `bugs_are_stored_deduplicated_and_recalled`, `contradictions_are_flagged_and_superseded_records_drop_out`
