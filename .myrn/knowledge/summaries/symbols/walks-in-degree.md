---
id: walks-in-degree
type: summary
status: unverified
title: walks::in_degree
source: myrn summarize
about: sym:crates/myrn-engine/src/retrieve/walks.rs#retrieve::walks::in_degree
anchors:
- sym:crates/myrn-engine/src/retrieve/walks.rs#retrieve::walks::in_degree@2f16f3cbff19f16b
---
# walks::in_degree

Counts incoming `calls` edges to the given id from sources not starting with `test:`, as a measure of how central the symbol is to non-test code; returns an i64 and errors on query failure.

## Connections

- **Location:** `crates/myrn-engine/src/retrieve/walks.rs:95-101`
- **Signature:** `pub fn in_degree(conn: &Connection, id: &str) -> Result<i64>`
- **In:** [[file-crates-myrn-engine-src-retrieve-walks-rs]]
- **Called by:** [[rank-rank]]
