---
id: has-edge-records
type: summary
status: unverified
title: has_edge
source: myrn summarize
about: sym:crates/myrn-engine/tests/records.rs#has_edge
anchors:
- sym:crates/myrn-engine/tests/records.rs#has_edge@7238e2732f048541
map_version: '2'
---
# has_edge

Returns whether the edges table holds a row with exactly the given src, rel and dst. Any query error is reported as absence.

## Behaviour

- matching src, rel and dst -> true
- no matching row -> false
- database error -> false, not a panic

## Terms

graph link, relation present, edge existence, triple lookup, connection check, relationship

## Connections

- **Location:** `crates/myrn-engine/tests/records.rs:119-128`
- **Signature:** `fn has_edge(e: &Engine, src: &str, rel: &str, dst: &str) -> bool`
- **In:** [[file-crates-myrn-engine-tests-records-rs]]
- **Calls:** [[engine-db]]
- **Tests:** `bugs_are_stored_deduplicated_and_recalled`, `contradictions_are_flagged_and_superseded_records_drop_out`
