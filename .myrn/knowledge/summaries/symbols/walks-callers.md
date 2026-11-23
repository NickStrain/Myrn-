---
id: walks-callers
type: summary
status: unverified
title: walks::callers
source: myrn summarize
about: sym:crates/myrn-engine/src/retrieve/walks.rs#retrieve::walks::callers
anchors:
- sym:crates/myrn-engine/src/retrieve/walks.rs#retrieve::walks::callers@c3c579cc9de3ad90
map_version: '2'
---
# walks::callers

Breadth-first walk over incoming calls edges from the target ids, up to max_depth hops and at most cap non-test callers. Test nodes are collected separately and not expanded.

## Behaviour

- max_depth 0 -> no edges are fetched; empty callers and tests, truncated false
- A caller found at BFS depth 1 is recorded with distance 1; its callers are recorded with distance 2
- Test node reached at depth d -> stored in tests with distance d+1, kept once at its first (shortest) distance, not counted against cap

## Terms

impact analysis, reverse call graph, transitive callers, upstream callers, breadth-first search, call depth, truncation, caller cap

## Connections

- **Location:** `crates/myrn-engine/src/retrieve/walks.rs:54-92`
- **Signature:** `pub fn callers( conn: &Connection, targets: &[String], max_depth: usize, cap: usize, ) -> Result<Reach>`
- **In:** [[file-crates-myrn-engine-src-retrieve-walks-rs]]
- **Calls:** [[graph-neighbors]]
- **Called by:** [[engine-impact]]
