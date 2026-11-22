---
id: walks-direct-tests
type: summary
status: unverified
title: walks::direct_tests
source: myrn summarize
about: sym:crates/myrn-engine/src/retrieve/walks.rs#retrieve::walks::direct_tests
anchors:
- sym:crates/myrn-engine/src/retrieve/walks.rs#retrieve::walks::direct_tests@e6f22b54ec5d65cb
---
# walks::direct_tests

Returns the test nodes that have an incoming `tests` edge to the given symbol id, i.e. tests that directly call it, discarding the edge metadata.

## Connections

- **Location:** `crates/myrn-engine/src/retrieve/walks.rs:38-43`
- **Signature:** `pub fn direct_tests(conn: &Connection, id: &str) -> Result<Vec<Node>>`
- **In:** [[file-crates-myrn-engine-src-retrieve-walks-rs]]
- **Calls:** [[graph-neighbors]]
- **Called by:** [[engine-symbol-card]], [[engine-impact]], [[rank-rank]], [[engine-map-body]]
