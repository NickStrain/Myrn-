---
id: has-edge-records
type: summary
status: unverified
title: has_edge
source: myrn summarize
about: sym:crates/myrn-engine/tests/records.rs#has_edge
anchors:
- sym:crates/myrn-engine/tests/records.rs#has_edge@7238e2732f048541
---
# has_edge

Returns true if the edges table has a row matching the given src, rel, and dst.

## Connections

- **Location:** `crates/myrn-engine/tests/records.rs:119-128`
- **Signature:** `fn has_edge(e: &Engine, src: &str, rel: &str, dst: &str) -> bool`
- **In:** [[file-crates-myrn-engine-tests-records-rs]]
- **Calls:** [[engine-db]]
- **Tests:** `bugs_are_stored_deduplicated_and_recalled`, `contradictions_are_flagged_and_superseded_records_drop_out`
