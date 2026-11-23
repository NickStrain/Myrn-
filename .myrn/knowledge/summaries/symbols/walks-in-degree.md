---
id: walks-in-degree
type: summary
status: unverified
title: walks::in_degree
source: myrn summarize
about: sym:crates/myrn-engine/src/retrieve/walks.rs#retrieve::walks::in_degree
anchors:
- sym:crates/myrn-engine/src/retrieve/walks.rs#retrieve::walks::in_degree@2f16f3cbff19f16b
map_version: '2'
---
# walks::in_degree

Counts incoming calls edges to the given symbol id, skipping sources whose id starts with test:. Returns an i64.

## Behaviour

- Symbol with no incoming calls edges -> 0
- Edges from a source id starting with test: are excluded, other sources count
- Counts edge rows, so a duplicate edge row adds one each time

## Terms

centrality, fan-in, caller count, popularity, importance, incoming calls, how central

## Connections

- **Location:** `crates/myrn-engine/src/retrieve/walks.rs:95-101`
- **Signature:** `pub fn in_degree(conn: &Connection, id: &str) -> Result<i64>`
- **In:** [[file-crates-myrn-engine-src-retrieve-walks-rs]]
- **Called by:** [[rank-rank]]
